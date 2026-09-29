export function formatAge(iso, now = Date.now()) {
	const then = new Date(iso).getTime();
	if (Number.isNaN(then)) return '-';
	const sec = Math.max(0, Math.floor((now - then) / 1000));
	if (sec < 60) return `${sec}s`;
	const min = Math.floor(sec / 60);
	if (min < 60) return `${min}min`;
	const hr = Math.floor(min / 60);
	if (hr < 24) return `${hr}h`;
	const day = Math.floor(hr / 24);
	if (day < 14) return `${day}d`;
	const wk = Math.floor(day / 7);
	if (wk < 8) return `${wk}wk`;
	return `${Math.floor(day / 30)}mo`;
}

export function taskCountLabel(count) {
	if (count === 0) return 'No matches.';
	if (count === 1) return '1 task';
	return `${count} tasks`;
}
