<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let config = invoke('get_config');
</script>

<h1>Hello world</h1>

{#await config}
	<p>Hold on...</p>
{:then c}
	<pre>
{JSON.stringify(c, null, 2)}
</pre>
{/await}
