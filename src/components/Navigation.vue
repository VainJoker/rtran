<template>
  <el-menu
   :default-active="activeIndex"
    class="el-menu-demo"
    mode="horizontal"
    @select="handleSelect"
  >
    <el-menu-item index="1">Online Translate</el-menu-item>
    <el-menu-item index="2" disabled>Offline Translate</el-menu-item>
  </el-menu>
  <div class="h-6" />
</template>

<script lang="ts" setup>
import { ref } from 'vue'

const activeIndex = ref('1')
const handleSelect = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
</script>

