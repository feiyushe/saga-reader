function currentDateText() {
	const now = new Date();
	const year = now.getFullYear();
	const month = now.getMonth() + 1;
	const day = now.getDate();

	const formattedMonth = String(month).padStart(2, '0');
	const formattedDay = String(day).padStart(2, '0');

	return `${year}-${formattedMonth}-${formattedDay}`;
}

export { currentDateText };
