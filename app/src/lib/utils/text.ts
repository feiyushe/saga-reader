function isTextEmpty(text: string | null) {
  return text == null || text.length === 0;
}

function format(template: string, variables: Record<string, string>) {
  return template.replace(/\{(\w+)\}/g, (match, key) => {
    return variables[key] !== undefined ? variables[key] : match;
  });
}

export { isTextEmpty, format };
