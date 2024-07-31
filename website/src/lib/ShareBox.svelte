<script lang="ts">
	import { onMount } from 'svelte';
	import { toast } from '@zerodevx/svelte-toast';
	import { errorToast } from '$lib/toastPresets';

	let dialog: HTMLDialogElement;
	// Used as a reference to prevent clicks on the dialog from closing it.
	let dialogInner: HTMLDivElement;
	let linkInput: HTMLInputElement;

	/// Tries to display the share box with the given link.
	/// Returns true if the box was opened, false if it was already open.
	export function open(link: string): boolean {
		if (dialog.open) {
			return false;
		}
		dialog.showModal();
		linkInput.value = link;
		return true;
	}

	function toClipBoard(): void {
		const url = linkInput.value;
		if (!url) {
			return;
		}

		navigator.clipboard
			.writeText(url)
			.then(() => {
				console.log('Copied to clipboard');
				close();
			})
			.catch((err) => {
				toast.push('Failed to copy to clipboard, do we have permissions?', errorToast);
				console.error('Failed to copy to clipboard', err);
				close();
			});
	}

	function close(): void {
		dialog.close();
	}

	onMount(() => {
		dialog.addEventListener('click', () => dialog.close());
		dialogInner.addEventListener('click', (e) => {
			e.stopPropagation();
		});
		linkInput.addEventListener('focus', () => linkInput.select());
	});
</script>

<dialog
	data-pw="share-dialog"
	class="border border-gray-400 bg-white min-w-[80%]"
	bind:this={dialog}
>
	<div class="dialog-inner w-full h-full p-2" bind:this={dialogInner}>
		<div class="flex justify-between mb-3">
			<span class="font-bold text-green-800">Your link is ready!</span>
			<button name="share-dialog-close-btn" on:click={close}>&#x2716</button>
		</div>
		<div class="flex">
			<input
				name="share-dialog-link-input"
				type="text"
				readonly
				class="grow border border-gray-200 p-2 mr-2"
				value=""
				bind:this={linkInput}
			/>
			<button
				name="share-dialog-copy-btn"
				class="font-bold py-2 px-4 bg-green-700 text-white hover:bg-green-900"
				on:click={toClipBoard}>Copy</button
			>
		</div>
	</div>
</dialog>
