<template>
  <div style="margin: 20px" />
  <el-form
    label-position="top"
    label-width="100px"
    :model="formLabelAlign"
    style="max-width: 460px"
  >
    <el-form-item label="Username">
      <el-input v-model="formLabelAlign.Username" />
    </el-form-item>
    <el-form-item label="Password">
      <el-input v-model="formLabelAlign.Password" />
    </el-form-item>
    <el-form-item>
    <el-button type="primary" @click="submit">Submit</el-button>
    </el-form-item>
  </el-form>
</template>

<script lang="ts" setup>
import { reactive} from 'vue'
import router from '../router'

const formLabelAlign = reactive({
  Username: '',
  Password: '',
})

function submit() {
  if (formLabelAlign.Username === 'VJ' && formLabelAlign.Password === '123qwe') {
    router.push('/index')
  } else {
    alert('Username or Password is incorrect');
  }
}
</script>

