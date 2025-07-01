export interface BarcodeScannerOptions {
  prefix?: string;
  suffix?: string;
  timeout?: number;
  shouldCapture?: () => boolean;
  barcodeCallback: (code: string) => void;
  target?: HTMLElement | Window;
}

export default class BarcodeScanner {
  private buffer = "";
  private lastEventTime = 0;
  private timer: ReturnType<typeof setTimeout> | null = null;
  private options: Required<BarcodeScannerOptions> & {
    prefixRegex: RegExp | null;
    suffixRegex: RegExp | null;
  };

  constructor({
    prefix = "",
    suffix = "",
    timeout = 50,
    shouldCapture = () => true,
    barcodeCallback,
    target = window,
  }: BarcodeScannerOptions) {
    if (typeof barcodeCallback !== "function") {
      throw new Error("barcodeCallback must be a function");
    }

    this.options = {
      prefix,
      suffix,
      timeout,
      shouldCapture,
      barcodeCallback,
      target,
      prefixRegex: prefix ? new RegExp(`^${prefix}`) : null,
      suffixRegex: suffix ? new RegExp(`${suffix}$`) : null,
    };

    this.init();
  }

  init() {
    this.options.target.addEventListener("keydown", this.handleKeydown);
  }

  private handleKeydown = (event: Event) => {
    if (!(event instanceof KeyboardEvent)) return;

    if (event.key.length !== 1) {
      return;
    }

    if (event.key === "Tab") {
      event.preventDefault();
    }

    const now = Date.now();

    if (now - this.lastEventTime > this.options.timeout) {
      this.buffer = "";
    }
    this.lastEventTime = now;
    this.buffer += event.key;

    if (this.timer) {
      clearTimeout(this.timer);
    }

    this.timer = setTimeout(() => {
      const { prefixRegex, suffixRegex } = this.options;
      if (
        this.options.prefixRegex &&
        !this.options.prefixRegex.test(this.buffer)
      ) {
        this.buffer = "";
        return;
      }
      if (
        this.options.suffixRegex &&
        !this.options.suffixRegex.test(this.buffer)
      ) {
        this.buffer = "";
        return;
      }

      let code = this.buffer;
      if (prefixRegex) code = code.replace(prefixRegex, "");
      if (suffixRegex) code = code.replace(suffixRegex, "");
      this.buffer = "";
      if (this.timer) {
        clearTimeout(this.timer);
        this.timer = null;
      }
      this.options.barcodeCallback(code);
    }, this.options.timeout);
  };

  destroy() {
    this.options.target.removeEventListener("keydown", this.handleKeydown);
    if (this.timer) {
      clearTimeout(this.timer);
    }
  }
}
