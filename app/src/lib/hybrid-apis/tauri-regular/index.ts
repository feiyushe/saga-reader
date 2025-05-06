import { invoke } from '@tauri-apps/api/core';

type InvokeArgs = Record<string, unknown> | number[] | ArrayBuffer | Uint8Array;

type Option<T> = T | null | undefined;

async function call<T>(methodName: string, args: InvokeArgs): Promise<T> {
	return invoke<T>(`plugin:feed-api|${methodName}`, args);
}

export type { InvokeArgs, Option };

export { call };
