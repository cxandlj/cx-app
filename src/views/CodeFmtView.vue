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

    <el-col :span="12" align="left">
        <el-button type="primary" @click="format()" style="margin-left:20px;">格式化</el-button>
        <el-button type="primary" @click="clear(1)">{{isDiff?'清空左侧':'清空'}}</el-button>
        <el-button type="primary" v-show="isDiff" @click="clear(2)">清空右侧</el-button>
        <el-button type="primary" @click="copy(1)">{{isDiff?'复制左侧':'复制'}}</el-button>
        <el-button type="primary" v-show="isDiff" @click="copy(2)">复制右侧</el-button>

    </el-col>
    <el-col :span="6">
        <el-button v-show="false" style="margin-left:10px;" type="primary" @click="compare">比较</el-button>
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
import prettier from 'prettier/standalone';
import parserBabel from 'prettier/parser-babel';
import parserHtml from 'prettier/parser-html';
import { invoke } from '@tauri-apps/api/tauri'
import { ElNotification } from 'element-plus'
let editor, diffEditor;

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
                    label:'html',
                    value:'html'
                },
                {
                    label:'angular',
                    value:'angular'
                },
                {
                    label:'rust',
                    value:'rust'
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
                case 'javascript':
                    editor.setValue(prettier.format(editor.getValue(),
                        {  parser: "babel-flow", plugins: [parserBabel] }));
                    break;
                case 'json':
                    editor.setValue(prettier.format(editor.getValue(),
                        { parser: "json-stringify" , plugins: [parserBabel] }));
                    break;
                case 'html':
                case 'angular':
                    editor.setValue(prettier.format(editor.getValue(),
                        { parser: "html" , plugins: [parserHtml] }));
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
        
        clear(type) {
            if (this.isDiff) {
                if (type == 1) {
                    diffEditor.getModel().original.setValue('');
                }
                else {
                    diffEditor.getModel().modified.setValue('');
                }
            }
            else {
                editor.setValue('');
            }
        },
        copy(type) {

            var txt = '';
            if (type == 1) {
                if (this.isDiff) {
                    txt = diffEditor.getModel().original.getValue();
                }
                else {
                    txt = editor.getValue();
                }
            }
            else {
                txt = diffEditor.getModel().modified.getValue();
            }

            invoke('copy_text', {
                txt: txt,
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
                            theme: 'vs-dark',
                            readOnly: false,
                            originalEditable:true
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
    },
    computed: {
        isDiff() {
            return this.activeName == 'diff';
        }
    }
}
</script>

<style scoped>

</style>