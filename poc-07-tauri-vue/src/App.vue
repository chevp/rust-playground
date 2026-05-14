<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const name = ref("Patrice");
const greeting = ref("");
const count = ref(0);

async function greet() {
  greeting.value = await invoke<string>("greet", { name: name.value });
}
</script>

<template>
  <main class="container">
    <h1>poc-07-tauri-vue</h1>
    <p class="subtitle">Vue 3 frontend, Rust backend, IPC via <code>invoke</code>.</p>

    <section>
      <h2>Counter (frontend only)</h2>
      <button @click="count++">count is {{ count }}</button>
    </section>

    <section>
      <h2>IPC roundtrip</h2>
      <input v-model="name" placeholder="Your name" />
      <button @click="greet">Greet from Rust</button>
      <p v-if="greeting" class="reply">{{ greeting }}</p>
    </section>
  </main>
</template>

<style>
:root {
  font-family: system-ui, sans-serif;
  color-scheme: light dark;
}
.container {
  max-width: 640px;
  margin: 0 auto;
  padding: 2rem;
}
section {
  margin-top: 1.5rem;
}
button {
  padding: 0.4rem 0.9rem;
  margin-left: 0.4rem;
  cursor: pointer;
}
input {
  padding: 0.4rem;
  font: inherit;
}
.reply {
  margin-top: 0.5rem;
  font-style: italic;
}
.subtitle {
  opacity: 0.7;
}
</style>
