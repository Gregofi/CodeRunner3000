<script lang="ts">
  import AlgoCard from "./AlgoCard.svelte";

  const algorithms = [
    {
      name: "Regular Expression to Finite Automaton",
      description:
        "Visualize the conversion of regular expressions to finite automata, including structures used in the process.",
      path: "regexes",
    },
  ];
</script>

<section class="mx-auto max-w-4xl flex flex-col items-center">
  {#each algorithms as { name, description, path }}
    <AlgoCard {name} {description} {path} />
  {/each}
</section>
