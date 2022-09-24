<script setup>
import { ref, onMounted, nextTick } from "vue";
import { cli, invoke } from "@tauri-apps/api";
import { emit, listen } from "@tauri-apps/api/event";

onMounted(() => {
  listen("set_focus", (event) => {
    // textField.value.value = '';
    // textField.value.focus();
    // clist.value = []
    // choosen.value = 0
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
  textField.value.value = '';
  textField.value.focus();
  clist.value = []
  choosen.value = 0
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
    <div class="bg-gray-50  flex h-full w-full shadow-md rounded-md space-x-2 p-2">
      <div class=" p-2 rounded-md bg-white flex flex-col space-y-2" data-tauri-drag-region>
        <div class="h-8 w-8 rounded-md p-1 bg-blue-500">
          <svg class=" h-full w-full text-white" viewBox="0 0 24 24">
            <path fill="currentColor"
              d="M13,19A1,1 0 0,0 14,20H16V22H13.5C12.95,22 12,21.55 12,21C12,21.55 11.05,22 10.5,22H8V20H10A1,1 0 0,0 11,19V5A1,1 0 0,0 10,4H8V2H10.5C11.05,2 12,2.45 12,3C12,2.45 12.95,2 13.5,2H16V4H14A1,1 0 0,0 13,5V19Z" />
          </svg>
        </div>
        <div class="h-8 w-8 rounded-md p-1 bg-gray-200">
          <svg class=" h-full w-full" viewBox="0 0 24 24">
            <path fill="currentColor"
              d="M15.6,5.29C14.5,5.19 13.53,6 13.43,7.11L13.18,10H16V12H13L12.56,17.07C12.37,19.27 10.43,20.9 8.23,20.7C6.92,20.59 5.82,19.86 5.17,18.83L6.67,17.33C6.91,18.07 7.57,18.64 8.4,18.71C9.5,18.81 10.47,18 10.57,16.89L11,12H8V10H11.17L11.44,6.93C11.63,4.73 13.57,3.1 15.77,3.3C17.08,3.41 18.18,4.14 18.83,5.17L17.33,6.67C17.09,5.93 16.43,5.36 15.6,5.29Z" />
          </svg>
        </div>
        <div class="h-8 w-8 rounded-md p-1 bg-gray-200">
          <svg class=" h-full w-full" viewBox="0 0 24 24">
            <path fill="currentColor"
              d="M11,18H13V16H11V18M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2M12,20C7.59,20 4,16.41 4,12C4,7.59 7.59,4 12,4C16.41,4 20,7.59 20,12C20,16.41 16.41,20 12,20M12,6A4,4 0 0,0 8,10H10A2,2 0 0,1 12,8A2,2 0 0,1 14,10C14,12 11,11.75 11,15H13C13,12.75 16,12.5 16,10A4,4 0 0,0 12,6Z" />
          </svg>
        </div>
      </div>
      <div class=" flex flex-col h-full w-full">
        <div class="flex justify-center items-center h-16 my-2 w-full">
          <input ref="textField" type="text" @input="text_field_change_event"
            class="p-2 rounded-md focus:outline-slate-200 font-mono w-full" />
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
  </div>
</template>