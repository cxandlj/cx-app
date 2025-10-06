<template>
  <el-row>
    <el-col :span="24" style="text-align:left;padding: 5px 20px ;">
         <el-button type="primary" @click="goHome()">返回主界面</el-button>
    </el-col>
    <el-col :span="8" justify="center" class="regex-col">
      <el-card class="box-card">
                <template #header>
                <div class="card-header">
                    <span>原始内容</span>
                    <el-button type="warning" @click="clearText">清空</el-button>
                    <!-- <el-button type="success" @click="matchText">导入</el-button> -->
                </div>
                </template>
                <el-input
        v-model="textIn"
        rows="20"
        type="textarea"
        placeholder="请输入原始内容"
        />
              </el-card>
       
    </el-col>
    <el-col :span="8" class="regex-col">
        <el-row :gutter="20">     
            <el-col :span="24">
              <el-card class="search-box-card">
                <template #header>
                <div class="card-header">
                    <span>查找</span>                    
                    <el-button type="success" @click="matchText">匹配</el-button>
                </div>
                </template>

                <el-row>
                    <el-col :span="24" style="padding: 0 0 5px 0;margin-top:-10px;">
                        <el-checkbox-group v-model="searchCheckList">
                        <el-checkbox v-for="item in regex_options.filter(p=>p.code=='i')" :key="item.code" :label="item.code">{{
                        item.text
                        }}
                        </el-checkbox>
                        </el-checkbox-group>
                    </el-col>
                    <el-col :span="3">
                        匹配正则
                    </el-col>
                    <el-col :span="21" style="padding: 0 5px;">
                        <el-input
                    v-model="textMatch"
                    rows="4"
                    type="textarea"
                    placeholder="请输入匹配正则"
                    />
                    </el-col>
                </el-row>
              </el-card>
            </el-col>
        </el-row>
        <el-row :gutter="20" style="margin-top:10px;">
            <el-col :span="24">
                <el-card class="replace-box-card">
                    <template #header>
                    <div class="card-header">
                        <span>替换</span>
                        <el-button type="success" @click="replaceText">替换</el-button>
                    </div>
                    </template>
                    <el-row>
                         <el-col :span="24" style="padding: 0 0 5px 0;margin-top:-10px;">
                        <el-checkbox-group v-model="replaceCheckList">
                        <el-checkbox v-for="item in regex_options" :key="item.code" :label="item.code">{{
                        item.text
                        }}
                        </el-checkbox>
                        </el-checkbox-group>
                    </el-col>
                    <el-col :span="3">
                        匹配正则
                    </el-col>
                    <el-col :span="21" style="padding: 0 5px;">
                        <el-input
                    v-model="textReplaceMatch"
                    rows="3"
                    type="textarea"
                    placeholder="请输入匹配正则"
                    />
                    </el-col>
                </el-row>               
                <el-row style="margin-top:10px;">
                    <el-col :span="3">
                        替换内容
                    </el-col>
                    <el-col :span="21" style="padding: 0 5px;">
                        <el-input
                    v-model="textReplace"
                    rows="3"
                    type="textarea"
                    placeholder="请输入替换内容"
                    />
                    </el-col>
                </el-row>
                </el-card>
                </el-col>
        </el-row>
    </el-col>
    <el-col :span="8" justify="center" class="regex-col">
    <el-card class="box-card">
                <template #header>
                <div class="card-header">
                    <span>处理结果</span>
                    <el-button type="success" @click="copyOut">复制结果</el-button>
                </div>
                </template>
                <el-input
        v-model="textOut"
        rows="20"
        readonly
        type="textarea"
        />
              </el-card>
    </el-col>
  </el-row>
</template>

<script>
import { ArrowLeftBold } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { ElNotification } from 'element-plus'
export default {
    data() {
        return {
            textIn: '',
            textMatch: '',
            textReplace: '',
            textReplaceMatch: '',
            textReplaceReplace:'',
            textOut: '',
            searchCheckList:['i'],
            replaceCheckList:['i','g'],
            regex_options: [
                {
                    code: 'i',
                    text: '不区分大小写',
                },
                {
                    code: 'g',
                    text:'全局匹配',
                }
            ]
        }
    },
    methods: {
        matchText() {
            invoke('match_text', {
                input: this.textIn,
                regexStr:this.textMatch,
                options:this.searchCheckList,
                })
                .then((res) =>
                    this.textOut=res
                )
                .catch((e) => console.error(e))
        },
        replaceText() {
            if (!this.textReplace) {
                
            }

            invoke('replace_text', {
                input: this.textIn,
                regexStr:this.textReplaceMatch,
                replaceStr: this.textReplace,
                options:this.replaceCheckList,
                })
                .then((res) =>
                    this.textOut=res
                )
                .catch((e) => console.error(e))
        },
        copyOut() {
            invoke('copy_text', {
                txt: this.textOut,
                })
                .then((res) =>
                     ElNotification({
                        message: '复制成功',
                         type: 'success',
                        duration:2000
                    })
                )
                .catch((e) => console.error(e))
        },
        clearText() {
          this.textIn=''  
        },
        goHome() {
            this.$router.push({name:'home'})
        }
    }
}
</script>

<style scoped>

.regex-col{
  height: 100%;
  padding: 5px;
}

.box-card{
    height: 540px;
}

.search-box-card{
    height: 230px;
}

.replace-box-card{
    height: 298px;
}

.card-title{
    display: block;
}
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
