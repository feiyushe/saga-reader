<script lang="ts">
    interface Props {
      href?: string;
      title?: string;
      text?: string;
    }

    const { href = '', title = undefined, text = '' }: Props = $props()
    let error = $state(false);

    function onerror() {
      error = true;
      console.warn(`image load error by MarkdownImg...${href}`);
    }
</script>

{#if !error}
    <img src={href} {title} alt={text} {onerror} />
{/if}
