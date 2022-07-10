<template>
  <Navigation />
  <el-input v-model="textarea" autosize clearable type="textarea" placeholder="请输入要翻译的内容" />
  <el-button type="primary" @click="tstranslate">翻译</el-button>
  <div id="result">{{result}}</div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import Navigation from '../components/Navigation.vue'
import { invoke } from '@tauri-apps/api'
const textarea = ref('')
let temp = '';

function tstranslate() : string {
  invoke('rstranslate', { word: textarea.value }).then(res => {
    temp = res as string;
    document.getElementById('result').innerHTML = temp;
  })
  return temp;
}

let result = tstranslate();

</script>

