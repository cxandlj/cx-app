<template>
    <el-row align="middle">
        <el-col :span="24" style="text-align:left;padding: 5px 20px ;">
         <el-button type="primary" @click="goHome()">返回主界面</el-button>
    </el-col>
        <el-col :span="2">
        <p style="margin-left: 10px">语言</p>
    </el-col>
    <el-col :span="4">
        <el-select 
        v-model="language"
        placeholder="请选择"
        @change="languageChange"
        >
        <el-option
            v-for="item in options"
            :key="item.value"
            :label="item.label"
            :value="item.value"
        />
        </el-select>
    </el-col>

    <el-col :span="2">
        <el-button type="primary" @click="format()">格式化</el-button>
    </el-col>
    <el-col :span="8">
        <!-- <el-switch
            v-model="isDiff"
            active-text="比较模式"
            inactive-text="普通模式"
            @change="isDiffChange"
        /> -->
        <el-button v-show="activeName=='diff'" style="margin-left:10px;" type="primary" @click="compare">比较</el-button>
    </el-col>
    <el-col :span="24"> 
        <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane label="普通模式" name="common">
                <div id="container" style="width:100%;height:450px;text-align:left;float: right;"></div>
            </el-tab-pane>
            <el-tab-pane label="比较模式" name="diff">                
                <div id="containerDiff" style="width:100%;height:450px;text-align:left;float: left;"></div>
            </el-tab-pane>
        </el-tabs>
    </el-col>
    </el-row>

</template>

<script>
import * as monaco from 'monaco-editor';
import { format as sqlFormat } from 'sql-formatter';
let editor,diffEditor;
export default {
    data() {
        return {           
            code:`console.log('Hello, world!')`,
            language:'json',
            options: [
                {
                    label:'json',
                    value:'json'
                },
                {
                    label:'sql',
                    value:'sql'
                },
                {
                    label:'javascript',
                    value:'javascript'
                },
                {
                    label:'rust',
                    value:'rust'
                },
                {
                    label:'html',
                    value:'html'
                },
                {
                    label:'csharp',
                    value:'csharp'
                }
            ],
            // editor:null,
            activeName:'common',
        }
    },

    mounted() {
        this.initEditor('common');
    },
    
    methods: {
        goHome() {
            this.$router.push({name:'home'})
        },
        format() {            
            switch (this.language) {
                case 'sql':
                    editor.setValue(sqlFormat(editor.getValue()));
                    break;     
                default:
                    editor.trigger("anyString", 'editor.action.formatDocument');
                    editor.setValue(editor.getValue());
                    break;
            }               
                
        },
        languageChange() {
            var newModel = monaco.editor.createModel('', this.language);
            editor.setModel(newModel);
        },
        isDiffChange(val) {
            if (val) {
                
            }
        },
        compare() {
            
        },
        handleClick(tab, event) {
            setTimeout(() => {
                this.initEditor(tab.props.name);
            }, 0);
        },
        initEditor(name) {
             switch (name) {
                case 'common':
                    if (!editor) {
                        document.getElementById('container').innerHTML = "";
                        editor = monaco.editor.create(document.getElementById('container'), {
                            value: '',
                            language: this.language,
                            theme:'vs-dark'
                        });
                    }
                    break;
                 case 'diff':
                    debugger
                    if (!diffEditor) {
                        document.getElementById('containerDiff').innerHTML = "";
                        diffEditor = monaco.editor.createDiffEditor(document.getElementById('containerDiff'), {
                            // enableSplitViewResizing: false
                            theme:'vs-dark'
                        });
        
                        var lhsModel = monaco.editor.createModel('original', this.language);
                        var rhsModel = monaco.editor.createModel('modified', this.language);
                        
                        diffEditor.setModel({
                            original: lhsModel,
                            modified: rhsModel
                        });
                    }
                    break;
            }

        }
    }
}
</script>

<style scoped>

</style>