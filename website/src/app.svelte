<script lang="ts">
  import Editor from './editor.svelte'
</script>

<main>
  <h1>tser</h1>
  <p>
   <a href="https://github.com/patr0nus/tser">GitHub</a>
  </p>
  <Editor />
</main>

<style>
  main {
    height: 100%;
    margin: 0 50px;
    display: flex;
    flex-direction: column;
  }
  h1 {
    margin-bottom: 0;
  }
</style>
