declare const __COMMIT_HASH__: string;
declare const __GIT_BRANCH__: string;
declare const __APP_VERSION__: string;
declare const __BUILD_DATE__: string;

declare namespace App {}

// Allow non-standard 'orient' attribute on range inputs (Firefox)
declare module "svelte/elements" {
  interface HTMLInputAttributes {
    orient?: string;
  }
}
