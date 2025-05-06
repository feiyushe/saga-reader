<script module>
	let previousContextMenuDisposer = () => {};
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';

	type MenuType = {
		name: string;
		onClick: () => void;
		displayText: string;
		class: string;
	};

	let { menus, children }: { menus: MenuType[]; children: Snippet } = $props();

	// showMenu is state of context-menu visibility
	let showMenu = $state(false);

	// pos is cursor position when right click occur
	let pos = $state({ x: 0, y: 0 });
	// menu is dimension (height and width) of context menu
	let menu = $state({ w: 0, h: 0 });
	// browser/window dimension (height and width)
	let browser = $state({ w: 0, h: 0 });

	function showContextMenu(e: MouseEvent) {
		if (previousContextMenuDisposer) previousContextMenuDisposer();
		previousContextMenuDisposer = () => (showMenu = false);

		e.preventDefault();
		e.stopPropagation();
		showMenu = true;
		browser = {
			w: window.innerWidth,
			h: window.innerHeight
		};
		pos = {
			x: e.clientX,
			y: e.clientY
		};
		// If bottom part of context menu will be displayed
		// after right-click, then change the position of the
		// context menu. This position is controlled by `top` and `left`
		// at inline style.
		// Instead of context menu is displayed from top left of cursor position
		// when right-click occur, it will be displayed from bottom left.
		if (browser.h - pos.y < menu.h) pos.y = pos.y - menu.h;
		if (browser.w - pos.x < menu.w) pos.x = pos.x - menu.w;
		return true;
	}

	function onHideMenu() {
		// To make context menu disappear when
		// mouse is clicked outside context menu
		showMenu = false;
	}

	function getContextMenuDimension(node: HTMLElement) {
		// This function will get context menu dimension
		// when navigation is shown => showMenu = true
		let height = node.offsetHeight;
		let width = node.offsetWidth;
		menu = {
			h: height,
			w: width
		};
	}

	function wrapMenuPressHandler(handler: () => void) {
		return (e: MouseEvent) => {
			e.preventDefault();
			e.stopPropagation();
			handler();
			showMenu = false;
			return true;
		};
	}
</script>

<svelte:body on:click={onHideMenu} />

{#if menus && menus.length !== 0}
	<div class="cursor-pointer" oncontextmenu={showContextMenu} role="menu" tabindex="0">
		{@render children()}
	</div>

	{#if showMenu}
		<nav use:getContextMenuDimension style="position: absolute; top:{pos.y}px; left:{pos.x}px">
			<div class="navbar" id="navbar">
				<ul>
					{#each menus as menu (menu.name)}
						{#if menu.name == 'hr'}
							<hr />
						{:else}
							<li>
								<button onclick={wrapMenuPressHandler(menu.onClick)}
									><i class={menu.class}></i>{menu.displayText}</button
								>
							</li>
						{/if}
					{/each}
				</ul>
			</div>
		</nav>
	{/if}

	<!-- <svelte:window on:contextmenu|preventDefault={showContextMenu}
    on:click={onHideMenu} /> -->

	<style>
		.navbar {
			display: inline-flex;
			border: 0.5px #999 solid;
			width: 150px;
			background-color: #fff;
			border-radius: 3px;
			overflow: hidden;
			flex-direction: column;
		}
		.navbar ul {
			margin: 6px;
		}
		ul li {
			display: block;
			list-style-type: none;
			width: 1fr;
		}
		ul li button {
			font-size: 1rem;
			color: #222;
			width: 100%;
			height: 40px;
			text-align: left;
			border: 0px;
			background-color: #fff;
		}
		ul li button:hover {
			color: #000;
			text-align: left;
			border-radius: 3px;
			background-color: #eee;
		}
		ul li button i {
			padding: 0px 5px 0px 5px;
		}
		ul li button i.fa-square {
			color: #fff;
		}
		ul li button:hover > i.fa-square {
			color: #eee;
		}
		ul li button:hover > i.warning {
			color: crimson;
		}
		ul li button.info:hover {
			color: navy;
		}
	</style>
{/if}
