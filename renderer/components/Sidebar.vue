<script setup lang="ts">
import type { ImageInfo, Shape } from '../types';

const props = defineProps<{
  images: ImageInfo[];
  currentImage: ImageInfo | null;
  shapes: Shape[];
  selectedShapeIndex: number | null;
  defaultLabelName: string;
  canSave: boolean;
  isEditMode: boolean;
}>();

const emit = defineEmits<{
  (e: 'select-image', img: ImageInfo): void;
  (e: 'refresh-images'): void;
  (e: 'update:defaultLabelName', value: string): void;
  (e: 'select-shape', index: number): void;
  (e: 'delete-shape', index: number): void;
  (e: 'save-annotations'): void;
  (e: 'shape-label-change'): void;
  (e: 'update:isEditMode', value: boolean): void;
}>();

const getColorByLabel = (label: string) => {
  let hash = 0;
  for (let i = 0; i < label.length; i++) {
    hash = label.charCodeAt(i) + ((hash << 5) - hash);
  }
  const h = Math.abs(hash) % 360; 
  const s = 80 + (Math.abs(hash) % 20); 
  const l = 45 + (Math.abs(hash) % 20); 
  return `hsl(${h}, ${s}%, ${l}%)`;
};

const updateDefaultLabelName = (event: Event) => {
  const target = event.target as HTMLInputElement;
  emit('update:defaultLabelName', target.value);
};
</script>

<template>
  <div class="sidebar">
    <div class="section">
      <div class="mode-toggle">
        <button 
          :class="['mode-btn', { active: !isEditMode }]" 
          @click="emit('update:isEditMode', false)"
        >创建模式</button>
        <button 
          :class="['mode-btn', { active: isEditMode }]" 
          @click="emit('update:isEditMode', true)"
        >编辑模式</button>
      </div>
    </div>

    <div class="section">
      <h3>图片列表</h3>
      <ul class="image-list">
        <li
          v-for="img in images"
          :key="img.name"
          @click="emit('select-image', img)"
          :class="{ active: currentImage?.name === img.name }"
        >
          {{ img.name }} <span v-if="img.has_annotation">✔</span>
        </li>
      </ul>
      <button class="save-btn" style="margin-top: 10px" @click="emit('refresh-images')">
        刷新列表
      </button>
    </div>

    <div class="section">
      <h3>默认标签</h3>
      <input
        :value="defaultLabelName"
        @input="updateDefaultLabelName"
        class="label-input"
        placeholder="输入分类标签"
      />
    </div>

    <div class="section">
      <h3>标注多边形 ({{ shapes.length }})</h3>
      <ul class="polygon-list">
        <li
          v-for="(shape, index) in shapes"
          :key="index"
          :class="{ active: selectedShapeIndex === index }"
          @click="emit('select-shape', index)"
        >
          <div class="shape-info">
            <span
              class="shape-indicator"
              :style="{ backgroundColor: getColorByLabel(shape.label) }"
            ></span>
            <input
              v-model="shape.label"
              class="inline-input"
              @change="emit('shape-label-change')"
              @click.stop
            />
          </div>
          <button class="delete-btn" @click.stop="emit('delete-shape', index)">
            删
          </button>
        </li>
      </ul>
    </div>

    <div class="actions">
      <button
        class="save-btn"
        @click="emit('save-annotations')"
        :disabled="!canSave"
      >
        保存标注 (JSON)
      </button>
    </div>
  </div>
</template>

<style scoped>
.sidebar {
  width: 300px;
  background-color: #f8f9fa;
  border-right: 1px solid #dee2e6;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 15px;
  overflow-y: auto;
  z-index: 10;
}

.mode-toggle {
  display: flex;
  gap: 10px;
  margin-bottom: 5px;
}

.mode-btn {
  flex: 1;
  padding: 8px 0;
  border: 1px solid #0d6efd;
  background-color: white;
  color: #0d6efd;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
}

.mode-btn.active {
  background-color: #0d6efd;
  color: white;
}

.section h3 {
  margin-top: 0;
  margin-bottom: 8px;
  font-size: 15px;
  color: #333;
}

.label-input {
  width: 100%;
  padding: 8px;
  box-sizing: border-box;
  border: 1px solid #ccc;
  border-radius: 4px;
}

ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}

.image-list li {
  padding: 10px;
  background: white;
  border: 1px solid #dee2e6;
  margin-bottom: 8px;
  cursor: pointer;
  border-radius: 4px;
  display: flex;
  justify-content: space-between;
}

.image-list li:hover {
  background-color: #e9ecef;
}

.image-list li.active {
  background-color: #e7f1ff;
  border-color: #0d6efd;
  color: #0a58ca;
  font-weight: bold;
}

.polygon-list li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 10px;
  background: white;
  border: 2px solid transparent;
  border-bottom: 1px solid #dee2e6;
  margin-bottom: 8px;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.polygon-list li.active {
  border-color: #0d6efd;
  background-color: #f1f7ff;
}

.shape-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.shape-indicator {
  display: inline-block;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 1px solid #666;
}

.inline-input {
  width: 120px;
  padding: 4px;
  border: 1px solid #ddd;
  border-radius: 3px;
}

.delete-btn {
  background-color: #dc3545;
  color: white;
  border: none;
  padding: 4px 8px;
  border-radius: 4px;
  cursor: pointer;
}

.delete-btn:hover {
  background-color: #c82333;
}

.actions {
  margin-top: auto;
}

.save-btn {
  width: 100%;
  padding: 12px;
  background-color: #198754;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  font-weight: bold;
}

.save-btn:hover {
  background-color: #157347;
}

.save-btn:disabled {
  background-color: #6c757d;
  cursor: not-allowed;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .sidebar {
    width: 100%;
    height: 35vh;
    border-right: none;
    border-bottom: 1px solid #dee2e6;
  }
}
</style>
