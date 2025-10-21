<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { browser } from '$app/environment';
	import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';
	import { arch, locale, platform, version } from '@tauri-apps/plugin-os';
	import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
	import { featuresApi } from '$lib/hybrid-apis/feed/impl';
	import { disableContextMenu } from '$lib/utils/dom';
	import { Switch } from '@skeletonlabs/skeleton-svelte';
	import type { AppConfig, LLMProviderType } from '$lib/hybrid-apis/feed/types';
	import { isTextEmpty } from '$lib/utils/text';
	import { getTheme, setTheme, type ThemePresets } from '$lib/themes';

	type PressedHandler = () => void;
	type CheckSwitchedHandler = () => void;
	type SelectionSelectedHandler = (value: string) => void;

	let appConfig: AppConfig | null = $state(null);

	// 应用版本与系统信息
	let appName = $state('');
	let appVersion = $state('');
	let engineVersion = $state('');
	let sysArch = $state('');
	let sysLocale = $state('');
	let sysPlatform = $state('');
	let sysVersion = $state('');

	// 主题
	let theme: ThemePresets = $state('light');
	let isDarkModeEnabled = $derived(theme !== 'light');

	// LLM配置信息
	let activedProviderType: null | LLMProviderType = $state(null);
	let llmFormOllamaURI: string | null = $state(null);
	let llmFormOllamaModelName: string | null = $state(null);
	let llmFormGLMKey: string | null = $state(null);
	let llmFormPlatformModelPath: string | null = $state(null);

	let llmFormOpenAILikeBaseURI: string | null = $state(null);
	let llmFormOpenAILikeModelName: string | null = $state(null);
	let llmFormOpenAILikeKey: string | null = $state(null);

	// LLM表单错误信息
	let llmFormErrorOllamaURI = $derived(isTextEmpty(llmFormOllamaURI));
	let llmFormErrorOllamaModelName = $derived(isTextEmpty(llmFormOllamaModelName));
	let llmFormErrorGLMKey = $derived(isTextEmpty(llmFormGLMKey));
	let llmFormErrorPlatformModelPath = $derived(isTextEmpty(llmFormPlatformModelPath));

	let llmFormErrorOpenAILikeBaseURI = $derived(isTextEmpty(llmFormOpenAILikeBaseURI));
	let llmFormErrorOpenAILikeModelName = $derived(isTextEmpty(llmFormOpenAILikeModelName));
	let llmFormErrorOpenAILikeKey = $derived(isTextEmpty(llmFormOpenAILikeKey));

	// LLM表单用户变更状态
	let llmFormChangedOllama = $derived.by(() => {
		if (!appConfig) return false;
		return (
			llmFormOllamaURI !== appConfig.llm.provider_ollama.endpoint.api_base_url ||
			llmFormOllamaModelName !== appConfig.llm.provider_ollama.endpoint.model
		);
	});
	let llmFormChangedGLM = $derived.by(() => {
		if (!appConfig) return false;
		return llmFormGLMKey !== appConfig.llm.provider_glm.api_key;
	});
	let llmFormChangedPlatform = $derived.by(() => {
		if (!appConfig) return false;
		return llmFormPlatformModelPath !== appConfig.llm.provider_platform.model_path;
	});
	let llmFormChangedOpenAILike = $derived.by(() => {
		if (!appConfig) return false;
		return (
			llmFormOpenAILikeBaseURI !== appConfig.llm.provider_openai.api_base_url ||
			llmFormOpenAILikeKey !== appConfig.llm.provider_openai.api_key ||
			llmFormOpenAILikeModelName !== appConfig.llm.provider_openai.model_name
		);
	});

	// LLM表单变更保存与还原操作函数
	function createSaveLLMFormAction(syncToAppConfig: () => boolean) {
		return async () => {
			if (!syncToAppConfig()) return;
			if (!appConfig) return;
			await updateAppConfig(appConfig);
		};
	}

	// LLM切换操作函数
	function createLLMSwitcherAction(
		providerType: LLMProviderType,
		formValidator: () => boolean,
		configUpdater: () => void
	) {
		return async () => {
			if (!appConfig) return;

			if (!formValidator()) {
				console.warn('createLLMSwitcherAction', `设置页LLM配置项校验不通过...${providerType}`);
			}

			appConfig.llm.active_provider_type = providerType;
			configUpdater();
			await updateAppConfig(appConfig);
		};
	}

	const switchToLLMOllama = createLLMSwitcherAction(
		'ollama',
		() => !llmFormErrorOllamaURI && !llmFormErrorOllamaModelName,
		() => {
			if (!appConfig) return;
			appConfig.llm.provider_ollama.endpoint.api_base_url = llmFormOllamaURI || '';
			appConfig.llm.provider_ollama.endpoint.model = llmFormOllamaModelName || '';
		}
	);
	const switchToLLMGLM = createLLMSwitcherAction(
		'glm',
		() => !llmFormErrorGLMKey,
		() => {
			if (!appConfig) return;
			appConfig.llm.provider_glm.api_key = llmFormGLMKey || '';
		}
	);
	const switchToLLMOpenAILike = createLLMSwitcherAction(
		'openai',
		() =>
			!llmFormErrorOpenAILikeKey &&
			!llmFormErrorOpenAILikeBaseURI &&
			!llmFormErrorOpenAILikeModelName,
		() => {
			if (!appConfig) return;
			appConfig.llm.provider_openai = {
				model_name: llmFormOpenAILikeModelName || '',
				api_base_url: llmFormOpenAILikeBaseURI || '',
				api_key: llmFormOpenAILikeKey || ''
			};
		}
	);
	const switchToLLMPlatform = createLLMSwitcherAction(
		'platform',
		() => !llmFormErrorPlatformModelPath,
		() => {
			if (!appConfig) return;
			appConfig.llm.provider_platform.model_path = llmFormPlatformModelPath || '';
		}
	);

	// LLM表单变更保存与还原操作函数
	const saveLLMFormOllama = createSaveLLMFormAction(() => {
		if (!appConfig || llmFormErrorOllamaURI || llmFormErrorOllamaModelName) return false;
		appConfig.llm.provider_ollama.endpoint.api_base_url = llmFormOllamaURI || '';
		appConfig.llm.provider_ollama.endpoint.model = llmFormOllamaModelName || '';
		return true;
	});

	function restoreLLMFormOllama() {
		if (!appConfig) return;
		llmFormOllamaURI = appConfig.llm.provider_ollama.endpoint.api_base_url;
		llmFormOllamaModelName = appConfig.llm.provider_ollama.endpoint.model;
	}

	const saveLLMFormGLM = createSaveLLMFormAction(() => {
		if (!appConfig || llmFormErrorGLMKey) return false;
		appConfig.llm.provider_glm.api_key = llmFormGLMKey || '';
		return true;
	});

	function restoreLLMFormGLM() {
		if (!appConfig) return;
		llmFormGLMKey = appConfig.llm.provider_glm.api_key;
	}

	const saveLLMFormOpenAILike = createSaveLLMFormAction(() => {
		if (
			!appConfig ||
			llmFormErrorOpenAILikeKey ||
			llmFormErrorOpenAILikeBaseURI ||
			llmFormErrorOpenAILikeModelName
		)
			return false;
		appConfig.llm.provider_openai.api_base_url = llmFormOpenAILikeBaseURI || '';
		appConfig.llm.provider_openai.api_key = llmFormOpenAILikeKey || '';
		appConfig.llm.provider_openai.model_name = llmFormOpenAILikeModelName || '';
		return true;
	});

	function restoreLLMFormOpenAILike() {
		if (!appConfig) return;
		llmFormOpenAILikeBaseURI = appConfig.llm.provider_openai.api_base_url;
		llmFormOpenAILikeKey = appConfig.llm.provider_openai.api_key;
		llmFormOpenAILikeModelName = appConfig.llm.provider_openai.model_name;
	}

	const saveLLMFormPlatform = createSaveLLMFormAction(() => {
		if (!appConfig || llmFormErrorPlatformModelPath) return false;
		appConfig.llm.provider_platform.model_path = llmFormPlatformModelPath || '';
		return true;
	});

	function restoreLLMFormPlatform() {
		if (!appConfig) return;
		llmFormPlatformModelPath = appConfig.llm.provider_platform.model_path;
	}

	// 应用行为配置
	let enableAutoStartUp = $state(false);
	let enableFrequencyUpdate = $state(false);
	let enableNotification = $state(false);

	// 应用行为设置变更处理函数 - 跟随系统自动启动
	async function onAutoStartUpSwitched() {
		const beforeChangedVal = await isEnabled();
		await (beforeChangedVal ? disable() : enable());
		enableAutoStartUp = await isEnabled();
	}

	async function updateAppConfig(ac: AppConfig) {
		await featuresApi.set_app_config(ac);
		appConfig = await featuresApi.get_app_config();
		afterAppConfigUpdated();
	}

	// 应用行为设置变更处理函数 - 后台更新频率
	async function onFrequencyUpdateSwitched() {
		if (!appConfig) return;
		appConfig.daemon.frequency_feeds_update = !appConfig.daemon.frequency_feeds_update;
		await updateAppConfig(appConfig);
	}

	// 应用行为设置变更处理函数 - 通知开关
	async function onNotificationSwitched() {
		if (!appConfig) return;
		appConfig.daemon.enable_notification = !appConfig.daemon.enable_notification;
		await updateAppConfig(appConfig);
	}

	// 线上使用说明
	function openGLMGuide() {
		featuresApi.open_article_external('https://bigmodel.cn');
	}

	// 内存AppConfig更新后的衍生执行代码
	function afterAppConfigUpdated() {
		if (!appConfig) return;
		enableFrequencyUpdate = appConfig.daemon.frequency_feeds_update;
		enableNotification = appConfig.daemon.enable_notification;
		activedProviderType = appConfig.llm.active_provider_type;
		llmFormOllamaURI = appConfig.llm.provider_ollama.endpoint.api_base_url;
		llmFormOllamaModelName = appConfig.llm.provider_ollama.endpoint.model;
		llmFormGLMKey = appConfig.llm.provider_glm.api_key;
		llmFormPlatformModelPath = appConfig.llm.provider_platform.model_path;
		llmFormOpenAILikeKey = appConfig.llm.provider_openai.api_key;
		llmFormOpenAILikeBaseURI = appConfig.llm.provider_openai.api_base_url;
		llmFormOpenAILikeModelName = appConfig.llm.provider_openai.model_name;
	}

	function switchTheme() {
		setTheme(theme === 'light' ? 'dark' : 'light');
		theme = getTheme();
	}

	async function selectLang(lang: string) {
		if (!appConfig) return;
		appConfig.llm.instruct.lang = lang;
		await updateAppConfig(appConfig);
	}

	if (browser) {
		getName().then((val) => (appName = val));
		getVersion().then((val) => (appVersion = val));
		getTauriVersion().then((val) => (engineVersion = val));
		locale().then((val) => (sysLocale = val || 'unknown-locale'));
		sysArch = arch();
		sysPlatform = platform();
		sysVersion = version();
		theme = getTheme();

		isEnabled().then((val) => (enableAutoStartUp = val));

		featuresApi.get_app_config().then((snapshot: AppConfig) => {
			appConfig = snapshot;
			afterAppConfigUpdated();
		});
	}
</script>

<div use:disableContextMenu class="cursor-default w-full h-screen overflow-hidden flex flex-col">
	<div class="flex-1 w-full h-full overflow-hidden flex flex-col">
		<div class="pt-4 pb-4 pl-8 pr-8">
			<h3 class="h3">{$_('settings.label')}</h3>
		</div>
		<hr class="hr ml-8 mr-8" />
		{#if !appConfig}
			<p class="pl-8 mt-4">{$_('settings.loading')}</p>
		{:else}
			<div class="flex-1 h-full overflow-y-auto pl-8 pr-8">
				<!-- section begin of theme config -->
				{@render SectionHeader($_('settings.section_llm_appearance.label'))}
				{@render CheckOption(
					$_('settings.section_llm_appearance.theme.label'),
					$_('settings.section_llm_appearance.theme.description'),
					isDarkModeEnabled,
					switchTheme
				)}

				{@render SectionEnd()}
				<!-- section end of theme config -->

				<!-- {@render SectionHeader("语言偏好")}
        {@render SectionEnd()} -->

				{@render SectionHeader($_('settings.section_llm_provider.label'))}
				<div class="flex flex-col space-y-2">
					<p class="type-scale-2 text-surface-400">
						{$_('settings.section_llm_provider.tip')}
					</p>

					<!-- section begin of ollama config -->
					<div
						class={`p-4 rounded-md border-2 ${activedProviderType === 'ollama' ? 'border-primary-500' : ''} w-full`}
					>
						<div
							class="cursor-pointer flex justify-between items-center gap-4"
							onclick={switchToLLMOllama}
							onkeypress={switchToLLMOllama}
							role="button"
							tabindex="0"
						>
							<p class="h5">
								{$_('settings.section_llm_provider.provider_ollama')}
							</p>
							<Switch name="llm_ollama" readOnly checked={activedProviderType === 'ollama'} />
						</div>

						<p class="mt-2 ml-0.5 type-scale-1 text-surface-400">
							<span>{$_('settings.section_llm_provider.provider_ollama_tip')}</span>
							<span
								onkeypress={featuresApi.download_ollama}
								onclick={featuresApi.download_ollama}
								role="link"
								tabindex="0"
								class="cursor-pointer text-primary-500"
								>{$_('settings.section_llm_provider.provider_ollama_sentence_1')}</span
							>。
						</p>
						{#if activedProviderType === 'ollama'}
							<hr class="hr mt-2 mb-2" />
							<label class={`label ${llmFormErrorOllamaURI ? 'text-red-500' : ''}`}>
								<span class="label-text"
									>{$_('settings.section_llm_provider.provider_ollama_sentence_2')}</span
								>
								<input
									class="input p-2"
									type="url"
									bind:value={llmFormOllamaURI}
									placeholder={$_('settings.section_llm_provider.provider_ollama_sentence_5')}
								/>
							</label>
							<label class={`label mt-2 ${llmFormErrorOllamaModelName ? 'text-red-500' : ''}`}>
								<span class="label-text"
									>{$_('settings.section_llm_provider.provider_ollama_sentence_3')}</span
								>
								<input
									class="input p-2"
									type="text"
									bind:value={llmFormOllamaModelName}
									placeholder={$_('settings.section_llm_provider.provider_ollama_sentence_4')}
								/>
							</label>

							{#if llmFormChangedOllama}
								{@render LLMGroupSavePanel(saveLLMFormOllama, restoreLLMFormOllama)}
							{/if}
						{/if}
					</div>
					<!-- section end of theme ollama -->

					<!-- section begin of glm config -->
					<div
						class={`p-4 rounded-md border-2 ${activedProviderType === 'glm' ? 'border-primary-500' : ''} w-full`}
					>
						<div
							class="cursor-pointer flex justify-between items-center gap-4"
							onclick={switchToLLMGLM}
							onkeypress={switchToLLMGLM}
							role="button"
							tabindex="0"
						>
							<p class="h5">
								{$_('settings.section_llm_provider.provider_glm')}
							</p>
							<Switch name="llm_glm" readOnly checked={activedProviderType === 'glm'} />
						</div>
						<p class="mt-2 ml-0.5 type-scale-1 text-surface-400">
							<span>{$_('settings.section_llm_provider.provider_glm_sentence_1')}</span>
							<span
								onkeypress={openGLMGuide}
								onclick={openGLMGuide}
								role="link"
								tabindex="0"
								class="cursor-pointer text-primary-500"
								>{$_('settings.section_llm_provider.provider_glm_sentence_2')}</span
							>。
						</p>
						{#if activedProviderType === 'glm'}
							<hr class="hr mt-2 mb-2" />
							<label class={`label ${llmFormErrorGLMKey ? 'text-red-500' : ''}`}>
								<span class="label-text">API KEY</span>
								<input
									class="input p-2"
									type="text"
									bind:value={llmFormGLMKey}
									placeholder={$_('settings.section_llm_provider.provider_glm_sentence_3')}
								/>
							</label>
							{#if llmFormChangedGLM}
								{@render LLMGroupSavePanel(saveLLMFormGLM, restoreLLMFormGLM)}
							{/if}
						{/if}
					</div>
					<!-- section end of glm config -->

					<!-- section begin of openai-like config -->
					<div
						class={`p-4 rounded-md border-2 ${activedProviderType === 'openai' ? 'border-primary-500' : ''} w-full`}
					>
						<div
							class="cursor-pointer flex justify-between items-center gap-4"
							onclick={switchToLLMOpenAILike}
							onkeypress={switchToLLMOpenAILike}
							role="button"
							tabindex="0"
						>
							<p class="h5">
								{$_('settings.section_llm_provider.provider_openai')}
							</p>
							<Switch name="llm_openai" readOnly checked={activedProviderType === 'openai'} />
						</div>
						<p class="mt-2 ml-0.5 type-scale-1 text-surface-400">
							<span>{$_('settings.section_llm_provider.provider_openai_sentence_1')}</span>
						</p>
						{#if activedProviderType === 'openai'}
							<hr class="hr mt-2 mb-2" />
							<label class={`label ${llmFormErrorOpenAILikeBaseURI ? 'text-red-500' : ''}`}>
								<span class="label-text">API URL</span>
								<input
									class="input p-2"
									type="url"
									bind:value={llmFormOpenAILikeBaseURI}
									placeholder={$_('settings.section_llm_provider.provider_openai_sentence_2')}
								/>
							</label>
							<label class={`mt-2 label ${llmFormErrorOpenAILikeKey ? 'text-red-500' : ''}`}>
								<span class="label-text">API KEY</span>
								<input
									class="input p-2"
									type="text"
									bind:value={llmFormOpenAILikeKey}
									placeholder={$_('settings.section_llm_provider.provider_openai_sentence_3')}
								/>
							</label>
							<label class={`mt-2 label ${llmFormErrorOpenAILikeModelName ? 'text-red-500' : ''}`}>
								<span class="label-text">Model Name</span>
								<input
									class="input p-2"
									type="text"
									bind:value={llmFormOpenAILikeModelName}
									placeholder={$_('settings.section_llm_provider.provider_openai_sentence_4')}
								/>
							</label>
							{#if llmFormChangedOpenAILike}
								{@render LLMGroupSavePanel(saveLLMFormOpenAILike, restoreLLMFormOpenAILike)}
							{/if}
						{/if}
					</div>
					<!-- section end of openai-like config -->

					<!-- <div
						class={`p-4 rounded-md border-2 ${activedProviderType === 'platform' ? 'border-primary-500' : ''} w-full`}
					>
						<div
							class="cursor-pointer flex justify-between items-center gap-4"
							onclick={switchToLLMPlatform}
							onkeypress={switchToLLMPlatform}
							role="button"
							tabindex="0"
						>
							<p class="h5">{$_('settings.section_llm_provider.provider_barton')}</p>
							<Switch name="llm_platform" readOnly checked={activedProviderType === 'platform'} />
						</div>
						<p class="mt-2 ml-0.5 type-scale-1 text-surface-400">
							{$_('settings.section_llm_provider.provider_barton_sentence_1')}
						</p>
						<hr class="hr mt-2 mb-2" />
						<label class={`label ${llmFormErrorPlatformModelPath ? 'text-red-500' : ''}`}>
							<span class="label-text"
								>{$_('settings.section_llm_provider.provider_barton_sentence_2')}</span
							>
							<input
								class="input p-2"
								type="text"
								bind:value={llmFormPlatformModelPath}
								placeholder={$_('settings.section_llm_provider.provider_barton_sentence_3')}
							/>
						</label>
						{#if llmFormChangedPlatform}
							{@render LLMGroupSavePanel(saveLLMFormPlatform, restoreLLMFormPlatform)}
						{/if}
					</div> -->
				</div>

				{@render SectionEnd()}

				{@render SectionHeader($_('settings.section_llm_instruct.label'))}
				{@render SelectOption(
					$_('settings.section_llm_instruct.lang.label'),
					$_('settings.section_llm_instruct.lang.description'),
					[
						{
							label: $_('settings.section_llm_instruct.lang.as_system'),
							value: 'system'
						},
						{
							label: $_('settings.section_llm_instruct.lang.english'),
							value: 'English'
						},
						{
							label: $_('settings.section_llm_instruct.lang.chinese'),
							value: 'Chinese'
						}
					],
					appConfig.llm.instruct.lang,
					selectLang
				)}
				{@render SectionEnd()}

				{@render SectionHeader($_('settings.section_app_behavior.label'))}
				{@render CheckOption(
					$_('settings.section_app_behavior.option_autostart.label'),
					$_('settings.section_app_behavior.option_autostart.description'),
					enableAutoStartUp,
					onAutoStartUpSwitched
				)}

				{@render CheckOption(
					$_('settings.section_app_behavior.option_scheduled_fetch.label'),
					$_('settings.section_app_behavior.option_scheduled_fetch.description'),
					enableFrequencyUpdate,
					onFrequencyUpdateSwitched
				)}

				{@render CheckOption(
					$_('settings.section_app_behavior.option_notification.label'),
					$_('settings.section_app_behavior.option_notification.description'),
					enableNotification,
					onNotificationSwitched
				)}

				{@render SectionEnd()}
				{@render SectionHeader($_('settings.section_users_support.label'))}
				<div class="flex space-x-4">
					{@render LinkButton(
						$_('settings.section_users_support.visit_home'),
						'https://aiqino.netlify.app?s=desktop'
					)}
					{@render LinkButton(
						$_('settings.section_users_support.feedback'),
						'https://github.com/sopaco/saga-reader/issues'
					)}
					{@render LinkButton(
						$_('settings.section_users_support.blogs'),
						'https://skyron.netlify.app/downloads/'
					)}
					{@render LinkButton(
						$_('settings.section_users_support.changelogs'),
						'https://github.com/sopaco/saga-reader/releases'
					)}
				</div>

				{@render SectionEnd()}
				{@render SectionHeader($_('settings.section_version.label'))}
				<div class="flex space-x-2 justify-items-center mb-8">
					<img class="w-36 h-36 object-contain" src="/favicon.png" alt="logo" />
					<div class="flex flex-col space-y-2">
						<p class="mt-3 type-scale-3">{appName}</p>
						<p class="type-scale-2">
							{`${$_('settings.section_version.ver_app')}：${appVersion}`}
						</p>
						<p class="type-scale-2">
							{`${$_('settings.section_version.ver_engine')}：${engineVersion}`}
						</p>
						<p class="type-scale-2">
							{`${$_('settings.section_version.ver_system')}：${sysPlatform} ${sysVersion}, ${sysArch}, ${sysLocale}`}
						</p>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>

{#snippet SectionHeader(text: string)}
	<div>
		<h5 class="pt-4 pb-2 h5">{text}</h5>
	</div>
{/snippet}

{#snippet CheckOption(
	label: string,
	description: string,
	checked: boolean,
	onCheckSwitched: CheckSwitchedHandler
)}
	<div
		class="flex justify-between items-center gap-4 pt-2 pb-1 cursor-pointer"
		onclick={onCheckSwitched}
		onkeypress={onCheckSwitched}
		role="button"
		tabindex="0"
	>
		<p>{label}</p>
		<Switch name="label" readOnly {checked} />
	</div>
	<p class="mb-2 type-scale-2 text-surface-400">{description}</p>
{/snippet}

{#snippet SelectOption(
	label: string,
	_description: string,
	selections: { value: string; label: string }[],
	selectedValue: string,
	onSelected: SelectionSelectedHandler
)}
	<div class="flex justify-between items-center gap-4 pt-2 pb-1">
		<p>{label}</p>
		<select
			class="select w-52 cursor-pointer"
			value={selectedValue}
			onchange={(e) => onSelected((e.target as HTMLSelectElement).value)}
		>
			{#each selections as s (s.value)}
				<option value={s.value}>{s.label}</option>
			{/each}
		</select>
	</div>
{/snippet}

{#snippet SectionEnd()}
	<hr class="hr mt-6 mb-2" />
{/snippet}

{#snippet LinkButton(label: string, link: string)}
	<button
		type="button"
		class="btn w-full mt-2 mb-2 preset-outlined-surface-500 hover:preset-filled-primary-500"
		onclick={() => featuresApi.open_article_external(link)}>{label}</button
	>
{/snippet}

{#snippet LLMGroupSavePanel(onSave: PressedHandler, onCancel: PressedHandler)}
	<div class="pt-4 flex justify-end space-x-2">
		<button type="button" class="btn preset-filled-primary-500" onclick={onSave}
			>{$_('common_dialog.save')}</button
		>
		<button type="button" class="btn preset-tonal-surface" onclick={onCancel}
			>{$_('common_dialog.cancel')}</button
		>
	</div>
{/snippet}
