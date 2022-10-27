<script lang="ts">
  import * as tser from "../../crates/tser_wasm/pkg";
  import sampleSource from "./sample.ts?raw";

  import CodeEditor from "./code-editor.svelte";
  import type { Language } from "./code-editor.svelte";
  import { Text } from "@codemirror/state"

  let targetLanguage: Language = "rust";

  let text = Text.of(sampleSource.split('\n'))
  $: result = (() => {
    try {
      const generatedCode = tser.generate_from_ts(text.sliceString(0), targetLanguage);
      return { content: generatedCode, ok: true };
    } catch (err) {
      return { content: err + "", ok: false };
    }
  })();

  $: resultText = Text.of(result.content.split('\n'));

</script>

<div class="editor">
  <div class="panel">
    <CodeEditor bind:text lang="typescript" />
  </div>
  <div class="splitter" />
  <div class="panel">
    <p>
      <select bind:value={targetLanguage}>
        <option value="rust">Rust</option>
        <option value="swift">Swift</option>
      </select>
    </p>
    <CodeEditor
      text={resultText}
      lang={targetLanguage}
      readonly={true}
    />
  </div>
</div>

<style>
  .editor {
    display: flex;
    flex: 1 1 0;
    align-items: stretch;
    margin-bottom: 20px;
    border: 1px solid lightgray;
  }
  p {
    margin: 5px;
  }
  @media only screen and (max-width: 768px) {
    .editor {
      flex-direction: column;
    }
  }
  .editor > .panel {
    flex: 1 1 0;
    display: flex;
    flex-direction: column;
  }
  .splitter {
    flex: 0 0 1px;
    background-color: lightgray;
  }
</style>
