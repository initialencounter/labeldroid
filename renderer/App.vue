<script setup lang="ts">
import { Menu } from '@tauri-apps/api/menu';
import { useRouter } from 'vue-router';
import { onMounted } from 'vue';

const router = useRouter();

onMounted(async () => {
  const menu = await Menu.new({
    items: [
      {
        id: 'home',
        text: 'Home',
        action: () => {
          router.push('/home');
        },
      },
    ],
  });

  // 如果某个窗口未显式创建菜单，或者未显式设置菜单，那么此菜单将被分配给它。
  menu.setAsAppMenu().then((res) => {
    console.log('menu set success', res);
  });
});
</script>

<template>
  <div class="main-container">
    <div class="content">
      <router-view></router-view>
    </div>
  </div>
</template>