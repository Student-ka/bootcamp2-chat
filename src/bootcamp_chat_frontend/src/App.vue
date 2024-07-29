<script lang=""ts">
import { ref } from 'vue';
import { bootcamp_chat_backend } from '../../declarations/bootcamp_chat_backend/';


export default {
  data() {
    return{
      newNote: "",
      notes: [] as string[]
    }
  },
  methods: {
    dodajNotatke() {
      bootcamp_chat_backend.add_note(this.newNote)
    }
    async pobierzNotatki() {
      this.notes = await bootcamp_chat_backend.get_notes()
    }
  },
  mounted(){
    this pobierzNotatki()
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <div>
    {{  notes }}
  </div>
  <div>
    <textarea v-model="newNote"></textarea><button @click="dodajNotatke">Dodaj notatke</button>
  </div>
  </main>
</template>
