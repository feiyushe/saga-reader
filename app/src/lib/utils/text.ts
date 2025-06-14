function isTextEmpty(text: string | null) {
	return text == null || text.length === 0;
}

function format(template: string, variables: Record<string, string>) {
	return template.replace(/\{(\w+)\}/g, (match, key) => {
		return variables[key] !== undefined ? variables[key] : match;
	});
}

function removeCodeBlockWrapper(str: string) {
	// 定义正则表达式来匹配开头的代码块包裹字符
	const startRegex = /^(```(?:\w+)?[\s\n]*)*/;
	// 定义正则表达式来匹配结尾的代码块包裹字符
	const endRegex = /(```)$/;
	// 去除开头的代码块包裹字符
	let result = str.replace(startRegex, '');
	// 去除结尾的代码块包裹字符
	result = result.replace(endRegex, '');
	return result;
}

export { isTextEmpty, format, removeCodeBlockWrapper };
