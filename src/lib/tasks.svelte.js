import { invoke } from '@tauri-apps/api/core';

export const taskState = $state({
	items: [],
	error: '',
	loading: true,
});

function message(err) {
	if (typeof err === 'string') return err;
	if (err && typeof err.message === 'string') return err.message;
	return 'Something went wrong';
}

export async function loadTasks() {
	taskState.loading = true;
	try {
		taskState.items = await invoke('list_tasks');
		taskState.error = '';
	} catch (err) {
		taskState.error = message(err);
	} finally {
		taskState.loading = false;
	}
}

async function mutate(command, args) {
	try {
		const result = await invoke(command, args);
		taskState.error = '';
		taskState.items = await invoke('list_tasks');
		return result;
	} catch (err) {
		taskState.error = message(err);
		throw err;
	}
}

export function createTask(title) {
	return mutate('create_task', { title });
}

export function updateTaskTitle(id, title) {
	return mutate('update_task_title', { id, title });
}

export function setTaskStatus(id, status) {
	return mutate('set_task_status', { id, status });
}

export function deleteTask(id) {
	return mutate('delete_task', { id });
}
