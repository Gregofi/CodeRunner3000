<script>
	import '../app.css';
</script>

<div class="flex flex-col h-full">
	<header
		class="border-b flex flex-row h-12 items-center
               justify-between justify-center px-20"
	>
		<span>Code Runner 3000</span>
		<h1>Run code in browser</h1>
		<nav>
			<a class="m-1 underline" href="/code">Code runner</a>
			<span class="m-1 underline text-gray-400 hover:cursor-not-allowed">Gadgets</span>
		</nav>
	</header>

	<slot />
</div>
