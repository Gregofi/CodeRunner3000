import { toast } from "@zerodevx/svelte-toast";

type ToastOpts = Parameters<typeof toast.push>[1];

export const errorToast: ToastOpts = {
  theme: {
    "--toastBackground": "rgba(196, 0, 0, 1.0)",
    "--toastColor": "white",
    "--toastProgressBackground": "rgba(255, 255, 255, 0.5)",
  },
  duration: 5000,
  dismissable: true,
};
