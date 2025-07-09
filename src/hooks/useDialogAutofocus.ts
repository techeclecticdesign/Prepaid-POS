import { useRef, useEffect } from "react";

export function useDialogAutofocus<T extends HTMLElement = HTMLInputElement>(
  open: boolean,
) {
  const ref = useRef<T>(null);

  const handleDialogEntered = () => {
    ref.current?.focus();
  };

  useEffect(() => {
    if (!open && document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
  }, [open]);

  return { ref, handleDialogEntered };
}
