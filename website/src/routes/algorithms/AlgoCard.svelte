<script lang="ts">
  const {
    name,
    description,
    path: link,
  }: {
    name: string;
    description: string;
    path: string;
  } = $props();
</script>

<a
  class="p-4 mt-4 border border-gray-200 rounded-lg shadow-md cursor-pointer hover:bg-gray-50 transition-colors"
  href={`algorithms/${link}`}
>
  <div><h3 class="font-bold text-lg">{name}</h3></div>
  <p class="algo-description">{description}</p>
</a>
