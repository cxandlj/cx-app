<template>
    <div class="common-layout">
        <el-container class="main-container">
            <el-header class="main-header">
                <el-image class="header-logo" :src="logo" :fit="fit" />
                <span class="header-title">云</span>
            </el-header>
            <el-container>
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
                            <el-menu-item index="regex">正则工具</el-menu-item>
                            <el-menu-item index="codeFmt"
                                >代码格式化</el-menu-item
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
                            style="height: 100%"
                        >
                            <keep-alive>
                                <el-tab-pane
                                    v-for="tab in tabs"
                                    :key="tab.name"
                                    :label="tab.title"
                                    :name="tab.name"
                                    style="height: 100%"
                                >
                                    <component :is="tab.component" />
                                </el-tab-pane>
                            </keep-alive>
                        </el-tabs>
                    </el-main>
                    <el-footer class="main-footer">AuWind</el-footer>
                </el-container>
            </el-container>
        </el-container>
    </div>
</template>

<script>
import logo from '@/assets/clound.png'
// import codeFmtView from '@/views/codeFmt.vue'
// import regexView from '@/views/regex.vue'
// import aboutView from '@/views/about.vue'

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
        }
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

<style>
.common-layout {
    height: 100%;
}
.main-container {
    height: 100%;
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