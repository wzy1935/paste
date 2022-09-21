<script setup>
import { ref, onMounted, nextTick } from "vue";
import { cli, invoke } from "@tauri-apps/api";
import { emit, listen } from "@tauri-apps/api/event";

onMounted(() => {
  listen("set_focus", (event) => {
    textField.value.value = '';
    textField.value.focus();
    clist.value = []
    choosen.value = 0
  });

  document.addEventListener("keydown", async function (e) {
    await key_down_event(e);
  })
});

let clist = ref([]);
let choosen = ref(0);
let textField = ref(null);

async function inject_string() {
  if (clist.value.length > 0 && choosen.value < clist.value.length) {
    await invoke("inject_string", {
      string: clist.value[choosen.value].content,
    });
  } else {
    await hide_window();
  }
}

async function hide_window() {
  await invoke("inject_string", {
    string: "",
  });
}

async function key_down_event(event) {
  if (event.code == "ArrowDown") {
    choosen.value =
      clist.value.length == 0
        ? 0
        : (clist.value.length + choosen.value + 1) % clist.value.length;
  } else if (event.code == "ArrowUp") {
    choosen.value =
      clist.value.length == 0
        ? 0
        : (clist.value.length + choosen.value - 1) % clist.value.length;
  } else if (event.code == 'Escape') {
    hide_window();
  } else if (event.code == 'Enter') {
    inject_string();
  }
}

async function text_field_change_event(event) {
  let content = event.target.value;
  let res = await invoke("search_content", { content: content });
  clist.value = res;
}

async function item_click(index) {
  choosen.value = index;
  inject_string();
}

</script>

<template>
  <div class="h-screen w-screen p-2">
    <div class="bg-gray-50 flex-col flex h-full w-full shadow-md rounded-md">
      <div class="flex justify-center items-center h-16 my-2 w-full">
        <input ref="textField" type="text" @input="text_field_change_event"
          class="p-2 rounded-md focus:outline-slate-200 font-mono w-full mx-8" />
      </div>
      <div class="overflow-y-auto">
        <div v-for="(item, index) in clist" :key="index" class="">
          <div :class="index == choosen ? ' ring-2' : ' text-gray-400'"
            class="bg-white rounded-md flex m-1 p-2 shadow-sm hover:shadow-md cursor-pointer"
            @click="() => item_click(index)">
            <div class="font-bold text-xs w-1/3">
              {{ item.name }}
            </div>
            <div class="truncate text-xs w-2/3">
              {{ item.content }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>