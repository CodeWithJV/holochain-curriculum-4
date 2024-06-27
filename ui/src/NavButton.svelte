<script>
  let button
  export let onClick = () => {}
</script>

<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<button
  on:click={onClick}
  style="
      width: 50px;
      height: 50px;
      padding: 0;
      background-color: transparent;
      border: 0;
    "
  class="button"
>
  <div
    style="
        position: relative;
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
      "
    bind:this={button}
  >
    <div style="position: absolute; width: 100%; height: 100%;">
      <slot />
    </div>
  </div>
</button>

<style>
  .button {
    cursor: pointer;
    transition: all 0.2s;
  }

  .button:hover {
    transform: scale(0.9);

    filter: brightness(50%);
  }
</style>
