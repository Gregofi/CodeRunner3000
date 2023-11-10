<script>
	import '../app.css';
</script>

<div class="flex flex-col h-full">
	<header
		class="border-b flex flex-col items-center
               justify-between justify-center sm:flex-row sm:px-10 sm:h-12"
	>
		<span class="text-lg"><tt>CodeRunner3000</tt></span>
		<h1>Run code in browser</h1>
		<nav>
			<a class="m-1 underline" href="/code">Code runner</a>
			<span class="m-1 underline text-gray-400 hover:cursor-not-allowed">Gadgets</span>
		</nav>
	</header>

	<slot />
</div>
