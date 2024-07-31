<script>
	import { SvelteToast } from '@zerodevx/svelte-toast';
	import '../app.css';
</script>

<div class="flex flex-col">
	<!-- DO NOT CHANGE THE HEIGHT!!! It is bound to the height of the body in code/+page.svelte,
    see #main div CSS rules -->
	<header
		class="border-b flex flex-col items-center
               justify-between sm:flex-row sm:px-4 h-12 align-stretch"
	>
		<span class="text-lg"><tt>CodeRunner3000</tt></span>
		<h1 class="hidden sm:inline">Run code in browser</h1>
		<nav class="justify-between hidden sm:flex">
			<div>
				<a class="m-1 underline" href="/code">Code runner</a>
				<span class="m-1 underline text-gray-400 hover:cursor-not-allowed">Gadgets</span>
			</div>
			<a href="https://github.com/Gregofi/CodeRunner3000"
				><img src="github-mark.svg" alt="GitHub" class="m-1 w-6 h-6" /></a
			>
		</nav>
	</header>

	<slot />
</div>
<SvelteToast />
