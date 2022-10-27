<script context="module" lang="ts">
  import { EditorView } from "@codemirror/view";

  import { keymap } from "@codemirror/view";
  import { minimalSetup } from "codemirror";
  import { indentWithTab } from "@codemirror/commands";
  import { EditorState, StateEffect, Text } from "@codemirror/state";

  import { javascript } from "@codemirror/lang-javascript";
  import { rust as rustLang } from "@codemirror/lang-rust";
  import type { Extension } from "@codemirror/state";
  import { StreamLanguage } from "@codemirror/language";
  import { swift as swiftMode } from "@codemirror/legacy-modes/mode/swift";

  export type Language = "typescript" | "swift" | "rust";

  const rust: Extension = rustLang();
  const swift: Extension = StreamLanguage.define(swiftMode);
  const typescript: Extension = javascript({ jsx: false, typescript: true });

  const languageExtensions = {
    rust,
    swift,
    typescript,
  };

  const cmTheme = EditorView.theme({
    "&": { height: "100%" },
    ".cm-content": {
        fontFamily: 'ui-monospace,SFMono-Regular,SF Mono,Menlo,Consolas,Liberation Mono,monospace',
        fontSize: '14px'
    },
  });
</script>

<script lang="ts">
  export let text: Text;
  export let lang: Language | undefined;
  export let readonly: boolean = false;

  let cmParent: Element | null = null;
  let cmEditor: EditorView | null = null;

  $: {
    if (cmParent != null && cmEditor == null) {
      cmEditor = new EditorView({
        parent: cmParent,
        state: EditorState.create({ doc: text, extensions }),
        dispatch(transaction) {
            cmEditor.update([transaction])
            if (transaction.docChanged && !cmEditor.state.doc.eq(text)) {
                text = transaction.newDoc
            }
        }
      });
    }
  }

  $: if (cmEditor != null && !cmEditor.state.doc.eq(text)) {
      cmEditor.update([cmEditor.state.update({
          changes: {
              from: 0,
              to: cmEditor.state.doc.length,
              insert: text
          }
      })])
  }

  $: extensions = [
    minimalSetup,
    keymap.of([indentWithTab]),
    cmTheme,
    EditorView.lineWrapping,
    lang && languageExtensions[lang],
    EditorState.readOnly.of(readonly),
]

  $: cmEditor?.dispatch({
    effects: StateEffect.reconfigure.of(extensions),
  });
</script>

<cm-parent bind:this={cmParent} />

<style>
  cm-parent {
    flex: 1 1 0;
    overflow-y: auto;
    overflow-x: hidden;
    overscroll-behavior: contain;
  }
  cm-parent > :global(.cm-editor) {
      width: 100%;
      height: 100%;
  }
</style>
