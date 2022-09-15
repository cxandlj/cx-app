<template>
  <el-row :gutter="20">
    <el-col v-for="(item,index) in apps" :key="index" :span="6" justify="center">
      <div class="app-content" @click="toApp(item)">
        <el-image :src="item.imgUrl" class="app-img"/>
        <span class="app-text">{{item.text}}</span>
      </div>
    </el-col>
  </el-row>
</template>

<script>
// @ is an alias to /src
import regexPng from '@/assets/regex.png'
import textFormat from '@/assets/text-format.png'
export default {
  name: 'HomeView',
  components: {},
  data() {
    return {
      apps: [
        {
          imgUrl: regexPng,
          text: '正则工具',
          url:'regex'
        },
        {
          imgUrl: textFormat,
          text: '代码格式化',
          url:'codeFmt'
        }
      ]
    }
  },
  methods:{
    toApp(data) {
      this.$router.push({name:data.url})
    }
  }
}
</script>

<style scoped>
.el-col {
  padding: 20px 0;
}

.app-content{
  display: inline-block;
  cursor: pointer;
}

.app-content .app-img{
  width:100px;
  height: 100px;
}
.app-content .app-text{
  font-weight: bold;
  display: block;
}
</style>
