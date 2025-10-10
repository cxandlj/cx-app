<template>
    <div class="common-layout">
        <el-container class="main-container">
            <el-header class="main-header">
                <el-image class="header-logo" :src="logo" :fit="fit" />
                <span class="header-title">云</span>
            </el-header>
            <el-container style="overflow-y: scroll">
                <el-aside class="main-aside"
                    ><el-menu
                        :default-active="activeMenu"
                        class="el-menu-vertical"
                        @select="handleMenuSelect"
                        background-color="#fafafa"
                        text-color="#666666"
                        active-text-color="#409EFF"
                    >
                        <el-sub-menu index="system">
                            <template #title>
                                <el-icon><Tools /></el-icon>
                                <span>常用工具</span>
                            </template>
                            <el-menu-item index="regex"
                                ><el-icon><tickets /></el-icon
                                ><span>正则工具</span></el-menu-item
                            >
                            <el-menu-item index="codeFmt"
                                ><el-icon><tickets /></el-icon
                                ><span>代码格式化</span></el-menu-item
                            >
                        </el-sub-menu>
                        <el-menu-item index="about">
                            <el-icon><drizzling /></el-icon>
                            <span>关于</span>
                        </el-menu-item>
                    </el-menu></el-aside
                >
                <el-container>
                    <el-main>
                        <el-tabs
                            v-model="activeTab"
                            type="card"
                            closable
                            @tab-remove="removeTab"
                            @tab-click="handleTabClick"
                            class="main-tabs"
                        >
                            <keep-alive>
                                <el-tab-pane
                                    v-for="tab in tabs"
                                    :key="tab.name"
                                    :label="tab.title"
                                    :name="tab.name"
                                    style="height: 100%; overflow-y: scroll"
                                >
                                    <component :is="tab.component" />
                                </el-tab-pane>
                            </keep-alive>
                        </el-tabs>
                    </el-main>
                    <el-footer class="main-footer"
                        >AuWind{{ version }}</el-footer
                    >
                </el-container>
            </el-container>
        </el-container>

        <el-dialog
            v-model="updateDialogVisible"
            title="更新进度"
            width="50%"
            :show-close="false"
        >
            <el-progress
                :text-inside="true"
                :stroke-width="26"
                :percentage="updateProgress"
            />
        </el-dialog>
    </div>
</template>

<script>
import logo from '@/assets/clound.png'
// 导入 getVersion 函数
import { getVersion } from '@tauri-apps/api/app'
import { invoke, Channel } from '@tauri-apps/api/core'
import { ElMessageBox } from 'element-plus'

const onEvent = new Channel()
export default {
    data() {
        return {
            activeTab: undefined,
            activeMenu: undefined,
            fit: 'fill',
            logo: logo,
            tabs: [],
            menuConfig: {
                regex: {
                    title: '正则工具',
                    component: async () => {
                        const com = await import('@/views/regex.vue')
                        return com.default
                    },
                },
                codeFmt: {
                    title: '代码格式化',
                    component: async () => {
                        const com = await import('@/views/codeFmt.vue')
                        return com.default
                    },
                },
                about: {
                    title: '关于',
                    component: async () => {
                        const com = await import('@/views/about.vue')
                        return com.default
                    },
                },
            },
            version: '',
            updateSize: 0,
            updateCurrentSize: 0,
            updateProgress: 0,
            updateDialogVisible: false,
        }
    },
    mounted() {
        // 获取并打印应用版本
        getVersion().then((version) => {
            this.version = `@v${version}`
        })
        let that = this
        onEvent.onmessage = (message) => {
            console.log(1, message)
            if (message.event == 'Started') {
                that.updateSize = message.data.contentLength
                that.updateCurrentSize = 0
                that.updateProgress = 0
                that.updateDialogVisible = true
                console.log(2, that.updateSize)
            } else if (message.event == 'Progress') {
                that.updateCurrentSize += message.data.chunkLength
                that.updateProgress = Math.floor(
                    (that.updateCurrentSize / that.updateSize) * 100
                )
                console.log(3, that.updateCurrentSize, that.updateProgress)
            } else if (message.event == 'Finished') {
                that.updateProgress = 100
                setTimeout(() => {
                    that.updateDialogVisible = false
                }, 500)
            }
        }

        invoke('fetch_update', {})
            .then((res) => {
                ElMessageBox.confirm('存在新版本，确认要更新吗?', '系统提示', {
                    confirmButtonText: '确定',
                    cancelButtonText: '取消',
                    type: 'info',
                }).then(() => {
                    console.log('installj')
                    invoke('install_update', { onEvent: onEvent })
                })
            })
            .catch((e) => console.error(e))
    },
    methods: {
        // 处理菜单选择
        async handleMenuSelect(index) {
            const config = this.menuConfig[index]
            if (!config) return

            // 检查标签页是否已存在
            const existingTab = this.tabs.find((tab) => tab.name === index)
            if (!existingTab) {
                this.tabs.push({
                    title: config.title,
                    name: index,
                    component: await config.component(),
                })
            }

            // 激活选中的标签页和菜单
            this.activeTab = index
            this.activeMenu = index
        },

        // 移除标签页
        removeTab(targetName) {
            // if (this.tabs.length <= 1) {
            //     ElMessage.warning('至少保留一个标签页')
            //     return
            // }

            const currentTabs = this.tabs
            let newActiveTab = this.activeTab

            if (newActiveTab === targetName) {
                currentTabs.forEach((tab, index) => {
                    if (tab.name === targetName) {
                        const nextTab =
                            currentTabs[index + 1] || currentTabs[index - 1]
                        if (nextTab) {
                            newActiveTab = nextTab.name
                        } else {
                            newActiveTab = null
                        }
                    }
                })
            }

            this.activeTab = newActiveTab
            this.activeMenu = newActiveTab
            this.tabs = currentTabs.filter((tab) => tab.name !== targetName)
        },

        // 处理标签页点击
        handleTabClick(tab) {
            this.activeMenu = tab.paneName
        },
    },
}
</script>

<style scoped>
.common-layout {
    height: 100%;
}
.main-container {
    height: 100%;
}
.main-tabs {
    height: 100%;
    display: flex;
    flex-direction: column;
}
.main-tabs:deep(.el-tabs__content) {
    flex: 1;
}
.main-header {
    background-color: #fcf5e8;
    color: #333;
    line-height: 50px;
    height: 50px;
    display: flex;
    padding: 4px 10px;
    align-items: center;
    column-gap: 10px;
}
.main-header .header-logo {
    height: 42px;
    width: 42px;
}
.main-header .header-title {
    color: #1296db;
}
.main-aside {
    background-color: #fafafa;
    color: #333;
    line-height: 160px;
    width: 200px;
}

.main-footer {
    background-color: #f5f7fa;
    color: #333;
    line-height: 30px;
    height: 30px;
    text-align: right;
    padding-right: 20px;
    font-size: 12px;
}
</style>