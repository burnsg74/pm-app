<script>
	import { onDestroy, onMount } from 'svelte';
	import { formatAge, taskCountLabel } from './lib/taskReport.js';
	import {
		createTask,
		deleteTask,
		loadTasks,
		setTaskStatus,
		taskState,
		updateTaskTitle,
	} from './lib/tasks.svelte.js';

	const LANES = [
		{ status: 'todo', label: 'TODO' },
		{ status: 'in_progress', label: 'In Progress' },
		{ status: 'done', label: 'Done' },
	];

	const modLabel =
		typeof navigator !== 'undefined' && /Mac|iPhone|iPad/.test(navigator.platform) ? '⌘' : 'Ctrl+';

	let now = $state(Date.now());
	let busy = $state(false);
	let selectedId = $state(null);
	let draft = $state('');
	let dialog = $state(null);
	let saveError = $state('');

	let lanes = $derived(
		LANES.map((lane) => ({
			...lane,
			tasks: taskState.items.filter((task) => task.status === lane.status),
		})),
	);

	let selected = $derived(taskState.items.find((task) => task.id === selectedId) ?? null);

	let ageTimer;

	function focusField(node) {
		node.focus();
		node.select();
	}

	onMount(() => {
		loadTasks();
		ageTimer = setInterval(() => {
			now = Date.now();
		}, 30000);
	});

	onDestroy(() => {
		clearInterval(ageTimer);
	});

	function isMod(event) {
		return event.metaKey || event.ctrlKey;
	}

	function openCreate() {
		draft = '';
		saveError = '';
		dialog = { mode: 'create' };
	}

	function openEdit(task) {
		selectedId = task.id;
		draft = task.title;
		saveError = '';
		dialog = { mode: 'edit', id: task.id };
	}

	function closeDialog() {
		dialog = null;
		draft = '';
		saveError = '';
	}

	function onWindowKeydown(event) {
		if (isMod(event) && event.key.toLowerCase() === 'n') {
			event.preventDefault();
			if (!dialog) openCreate();
			return;
		}

		if (dialog) {
			if (event.key === 'Escape') closeDialog();
			return;
		}

		if (!selected || busy) return;

		if (isMod(event) && event.key.toLowerCase() === 'e') {
			event.preventDefault();
			openEdit(selected);
		} else if (isMod(event) && event.key === 'Enter' && event.shiftKey) {
			event.preventDefault();
			markDone();
		} else if (isMod(event) && event.key === 'Enter') {
			event.preventDefault();
			toggleStart();
		} else if (isMod(event) && event.key === 'Backspace') {
			event.preventDefault();
			removeSelected();
		}
	}

	async function saveDialog(event) {
		event.preventDefault();
		const title = draft.trim();
		if (!title || busy || !dialog) return;
		busy = true;
		saveError = '';
		try {
			if (dialog.mode === 'create') {
				const created = await createTask(title);
				selectedId = created.id;
			} else {
				await updateTaskTitle(dialog.id, title);
			}
			closeDialog();
		} catch {
			saveError = taskState.error || 'Could not save the task';
		} finally {
			busy = false;
		}
	}

	async function toggleStart() {
		if (!selected || busy || selected.status === 'done') return;
		const next = selected.status === 'in_progress' ? 'todo' : 'in_progress';
		busy = true;
		try {
			await setTaskStatus(selected.id, next);
		} catch {
			// taskState.error is shown on the board
		} finally {
			busy = false;
		}
	}

	async function markDone() {
		if (!selected || busy || selected.status === 'done') return;
		busy = true;
		try {
			await setTaskStatus(selected.id, 'done');
		} catch {
			// taskState.error is shown on the board
		} finally {
			busy = false;
		}
	}

	async function removeSelected() {
		if (!selected || busy) return;
		const id = selected.id;
		busy = true;
		try {
			await deleteTask(id);
			if (selectedId === id) selectedId = null;
		} catch {
			// taskState.error is shown on the board
		} finally {
			busy = false;
		}
	}
</script>

<svelte:window onkeydown={onWindowKeydown} />

<div class="workspace">
	<div class="report">
		<div class="toolbar">
			<button type="button" onclick={openCreate}>New task <kbd>{modLabel}N</kbd></button>
			{#if selected}
				<button type="button" onclick={() => openEdit(selected)} disabled={busy}>
					Edit <kbd>{modLabel}E</kbd>
				</button>
				{#if selected.status !== 'done'}
					<button type="button" onclick={toggleStart} disabled={busy}>
						{selected.status === 'in_progress' ? 'Stop' : 'Start'} <kbd>{modLabel}↩</kbd>
					</button>
					<button type="button" onclick={markDone} disabled={busy}>Done <kbd>{modLabel}⇧↩</kbd></button>
				{/if}
				<button type="button" onclick={removeSelected} disabled={busy}>Delete <kbd>{modLabel}⌫</kbd></button>
			{/if}
		</div>

		{#if taskState.error && !dialog}
			<p class="error" role="alert">{taskState.error}</p>
		{/if}

		{#if taskState.loading}
			<p class="dim">Loading tasks</p>
		{:else}
			{#each lanes as lane (lane.status)}
				<section class="lane" aria-labelledby="lane-{lane.status}">
					<h2 id="lane-{lane.status}">{lane.label}</h2>
					{#if lane.tasks.length > 0}
						<table>
							<thead>
								<tr>
									<th scope="col" class="id">ID</th>
									<th scope="col" class="age">Age</th>
									<th scope="col">Description</th>
								</tr>
							</thead>
							<tbody>
								{#each lane.tasks as task (task.id)}
									<tr
										class={lane.status}
										class:selected={selectedId === task.id}
										aria-selected={selectedId === task.id}
										tabindex="0"
										onclick={() => (selectedId = task.id)}
										onkeydown={(event) => {
											if (event.key === 'Enter') openEdit(task);
										}}
									>
										<td class="id">{task.id}</td>
										<td class="age">{formatAge(task.created_at, now)}</td>
										<td>{task.title}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					{/if}
					<p class="summary dim">{taskCountLabel(lane.tasks.length)}</p>
				</section>
			{/each}
		{/if}
	</div>

	{#if dialog}
		<div class="backdrop">
			<button type="button" class="scrim" aria-label="Close dialog" onclick={closeDialog}></button>
			<div class="dialog" role="dialog" tabindex="-1" aria-modal="true" aria-labelledby="dialog-title">
				<h2 id="dialog-title">{dialog.mode === 'create' ? 'New task' : 'Edit task'}</h2>
				<form onsubmit={saveDialog}>
					<label for="task-title">Title</label>
					<input
						id="task-title"
						type="text"
						bind:value={draft}
						{@attach focusField}
						autocomplete="off"
						spellcheck="false"
					/>
					{#if saveError}
						<p class="error" role="alert">{saveError}</p>
					{/if}
					<div class="dialog-actions">
						<button type="button" onclick={closeDialog}>Cancel</button>
						<button type="submit" disabled={busy || draft.trim() === ''}>Save</button>
					</div>
					<p class="hint dim">Enter to save · Esc to close</p>
				</form>
			</div>
		</div>
	{/if}
</div>

<style>
	.workspace {
		display: flex;
		flex-direction: column;
		flex: 1;
		min-height: 0;
		font-size: 14px;
	}

	.report {
		flex: 1;
		min-height: 0;
		overflow: auto;
		padding: 0.75rem 1rem 1rem;
	}

	.toolbar {
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
		margin-bottom: 1rem;
	}

	kbd {
		margin-left: 0.35rem;
		font-family: inherit;
		font-size: 0.85em;
		opacity: 0.75;
	}

	.error {
		margin: 0 0 0.75rem;
		color: var(--terminal-glow-color);
	}

	.lane {
		margin-bottom: 1.25rem;
	}

	.lane h2 {
		margin: 0 0 0.35rem;
		color: var(--terminal-glow-color);
		font-size: 0.85rem;
		font-weight: bold;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	table {
		width: auto;
		border-collapse: collapse;
	}

	th,
	td {
		border: none;
		background: transparent;
		color: inherit;
		padding: 0.05rem 1.25rem 0.05rem 0;
		font-weight: normal;
		text-transform: none;
	}

	th {
		color: var(--terminal-dim);
		border-bottom: 1px solid #333;
		padding-bottom: 0.2rem;
	}

	.id {
		text-align: right;
		font-variant-numeric: tabular-nums;
		padding-right: 1rem;
		min-width: 2.5rem;
	}

	.age {
		color: var(--terminal-dim);
		font-variant-numeric: tabular-nums;
		white-space: nowrap;
	}

	th.age {
		text-align: left;
	}

	tr.in_progress td {
		color: #7dff9a;
	}

	tr.in_progress td.age {
		color: #4ecf78;
	}

	tr.done td {
		color: var(--terminal-dim);
	}

	tr.selected td {
		background: #161616;
	}

	tr {
		cursor: pointer;
	}

	tr:focus {
		outline: 1px solid var(--terminal-fg);
		outline-offset: 2px;
	}

	.summary {
		margin: 0.35rem 0 0;
	}

	.backdrop {
		position: fixed;
		inset: 0;
		z-index: 20;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	button.scrim {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		margin: 0;
		padding: 0;
		border: none;
		background: rgba(0, 0, 0, 0.72);
		text-transform: none;
	}

	button.scrim:hover,
	button.scrim:focus {
		background: rgba(0, 0, 0, 0.72);
		color: transparent;
		outline: none;
	}

	.dialog {
		position: relative;
		z-index: 1;
		width: min(32rem, calc(100% - 2rem));
		padding: 1rem 1.1rem 0.85rem;
		border: 1px solid var(--terminal-border);
		background: #000;
		box-shadow: 0 18px 50px rgba(0, 0, 0, 0.55);
	}

	.dialog h2 {
		margin: 0 0 0.75rem;
		font-size: 1rem;
		letter-spacing: 0.06em;
		text-transform: uppercase;
	}

	.dialog label {
		display: block;
		margin-bottom: 0.35rem;
		text-transform: uppercase;
		font-size: 0.85rem;
	}

	.dialog-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.4rem;
		margin-top: 0.75rem;
	}

	.hint {
		margin: 0.45rem 0 0;
		font-size: 12px;
	}
</style>
