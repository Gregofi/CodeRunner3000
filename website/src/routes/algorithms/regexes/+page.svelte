<script lang="ts">
  import {
    Parser,
    determinize,
    glushkov,
    faToDot,
  } from "@gregofi1/regex-tooling";
  import { instance } from "@viz-js/viz";
  import { onMount } from "svelte";

  let regexInput = $state("ab*d?c*");
  let svgNFAWrapper: HTMLDivElement;
  let svgDFAWrapper: HTMLDivElement;

  const conversionAlgorithms = [
    { text: "Glushkov", id: "glushkov" },
    //{ name: "Thompson", value: "thompson" },
    //{ name: "Powerset", value: "powerset" }
  ];

  let work = async () => {
    try {
      const parser = new Parser(regexInput);
      const ast = parser.parse();
      const nfa = glushkov(ast);
      const nfaDot = faToDot(nfa);
      const dfa = determinize(nfa);
      const dfaDot = faToDot(dfa);

      const dot = await instance();
      const nfaSvg = await dot.renderString(nfaDot, { format: "svg" });
      svgNFAWrapper.innerHTML = nfaSvg;
      const dfaSvg = await dot.renderString(dfaDot, { format: "svg" });
      svgDFAWrapper.innerHTML = dfaSvg;
    } catch (error) {
      console.error("Error processing regex:", error);
    }
  };

  onMount(() => {
    work();
  });
</script>

<div class="flex flex-col mt-4 p-4 max-w-5xl mx-auto">
  <h1 class="text-2xl font-bold">
    Regular Expression Compiler to Finite Automater
  </h1>
  <p class="text-gray-600">
    Enter a regular expression to visualize its NFA and DFA. We only support
    limited features of regular expression, namely
  </p>
  <ul class="text-left text-gray-600">
    <li>- Implicit Concatenation</li>
    <li>- Union (<code>|</code>)</li>
    <li>- Kleene Star and Plus (e.g., <code>a*</code>, <code>a+</code>)</li>
    <li>- Optional (using <code>?</code>)</li>
    <li>- Grouping (using parentheses)</li>
  </ul>

  <div class="flex">
    <h2 class="mr-2">Conversion Algorithm from Regex to NFA/eNFA:</h2>
    <select>
      {#each conversionAlgorithms as algorithm}
        <option value={algorithm.id}>{algorithm.text}</option>
      {/each}
    </select>
  </div>

  <div class="mx-auto mt-8 mb-4">
    <input
      type="text"
      placeholder="Enter something..."
      class="w-full max-w-md text-lg px-4 py-3 font-mono border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500"
      oninput={work}
      bind:value={regexInput}
    />
  </div>

  <div>
    <h2 class="text-xl font-semibold mb-4 text-center">NFA</h2>
    <div bind:this={svgNFAWrapper}></div>
  </div>

  <hr class="my-4" />

  <div>
    <h2 class="text-xl font-semibold mb-4 text-center">DFA</h2>
    <div bind:this={svgDFAWrapper}></div>
  </div>
</div>
