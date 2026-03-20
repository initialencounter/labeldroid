<script setup lang="ts">
import { ref, computed, watch } from 'vue';
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

const cachedLabels = ref<Set<string>>(new Set());

watch(() => props.shapes, (newShapes) => {
  newShapes.forEach(s => cachedLabels.value.add(s.label));
}, { immediate: true, deep: true });

const uniqueLabels = computed(() => Array.from(cachedLabels.value));

const searchQuery = ref('');
const lastSearchIndex = ref(-1);

watch(searchQuery, () => {
  lastSearchIndex.value = -1;
});

const emit = defineEmits<{
  (e: 'select-image', img: ImageInfo): void;
  (e: 'refresh-images'): void;
  (e: 'update:defaultLabelName', value: string): void;
  (e: 'select-shape', index: number | null): void;
  (e: 'delete-shape', index: number): void;
  (e: 'save-annotations'): void;
  (e: 'delete-annotations'): void;
  (e: 'shape-label-change'): void;
  (e: 'update:isEditMode', value: boolean): void;
}>();

const handleSearch = () => {
  if (!searchQuery.value) {
    lastSearchIndex.value = -1;
    return;
  }
  const query = searchQuery.value.toLowerCase();
  let targetIndex = props.images.findIndex((img, idx) => idx > lastSearchIndex.value && img.name.toLowerCase().includes(query));
  
  if (targetIndex === -1) {
    targetIndex = props.images.findIndex(img => img.name.toLowerCase().includes(query));
  }

  if (targetIndex !== -1) {
    lastSearchIndex.value = targetIndex;
    const targetImg = props.images[targetIndex];
    emit('select-image', targetImg);
    setTimeout(() => {
      const el = document.getElementById('img-item-' + targetIndex);
      if (el) {
        el.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }
    }, 50);
  }
};

const getVocColormap = () => {
  const cmap = [];
  for (let i = 0; i < 256; i++) {
    let r = 0, g = 0, b = 0;
    let c = i;
    for (let j = 0; j < 8; j++) {
      r = r | (((c >> 0) & 1) << (7 - j));
      g = g | (((c >> 1) & 1) << (7 - j));
      b = b | (((c >> 2) & 1) << (7 - j));
      c = c >> 3;
    }
    cmap.push(`rgb(${r}, ${g}, ${b})`);
  }
  return cmap;
};

const vocColormap = getVocColormap();

const jumpToNextUnlabeled = () => {
  if (props.images.length === 0) return;
  
  let startIndex = 0;
  if (props.currentImage) {
    startIndex = props.images.findIndex(img => img.name === props.currentImage!.name);
  }
  
  let targetIndex = props.images.findIndex((img, idx) => idx > startIndex && !img.has_annotation);
  
  if (targetIndex === -1) {
    targetIndex = props.images.findIndex(img => !img.has_annotation);
  }
  
  if (targetIndex !== -1 && targetIndex !== startIndex) {
    emit('select-image', props.images[targetIndex]);
    setTimeout(() => {
      const el = document.getElementById('img-item-' + targetIndex);
      if (el) {
        el.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }
    }, 50);
  }
};

const getColorByLabel = (label: string) => {
  const labelsArray = uniqueLabels.value;
  let index = labelsArray.indexOf(label);
  if (index === -1) {
    index = labelsArray.length; // 新标签预判
  }
  // labelme 中 label_id 默认从 1 开始（跳过黑色）
  const labelId = (1 + index) % vocColormap.length;
  return vocColormap[labelId];
};

const updateDefaultLabelName = (event: Event) => {
  const target = event.target as HTMLInputElement;
  emit('update:defaultLabelName', target.value);
};
</script>

<template>
  <div class="sidebar">
    <div class="section">
      <button 
        :class="['mode-btn', 'single-toggle', { active: isEditMode }]" 
        @click="emit('update:isEditMode', !isEditMode)"
      >
        当前模式: {{ isEditMode ? '编辑' : '创建' }} (点击切换)
      </button>
    </div>

    <div class="section scrollable">
      <h3>图片列表</h3>
      <input
        v-model="searchQuery"
        class="label-input search-input"
        placeholder="搜索图片 (回车跳转)..."
        @keydown.enter="handleSearch"
      />
      <ul class="image-list">
        <li
          v-for="(img, index) in images"
          :key="img.name"
          :id="'img-item-' + index"
          @click="emit('select-image', img)"
          :class="{ active: currentImage?.name === img.name }"
        >
          {{ img.name }} <span v-if="img.has_annotation">✔</span>
        </li>
      </ul>
      <div style="display: flex; gap: 10px; margin-top: 10px;">
        <button class="save-btn" style="flex: 1;" @click="emit('refresh-images')">
          刷新列表
        </button>
        <button class="save-btn" style="flex: 1; background-color: #0d6efd;" @click="jumpToNextUnlabeled">
          下一未标注
        </button>
      </div>
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

    <div class="section scrollable">
      <h3>标注多边形 ({{ shapes.length }})</h3>
      <ul class="polygon-list">
        <li
          v-for="(shape, index) in shapes"
          :key="index"
          :class="{ active: selectedShapeIndex === index }"
          @click="emit('select-shape', selectedShapeIndex === index ? null : index)"
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
            <select
              v-if="uniqueLabels.length > 0"
              v-model="shape.label"
              class="inline-select"
              @change="emit('shape-label-change')"
              @click.stop
            >
              <option :value="shape.label" disabled hidden></option>
              <option v-for="lbl in uniqueLabels" :key="lbl" :value="lbl">{{ lbl }}</option>
            </select>
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
      <button
        class="delete-json-btn"
        @click="emit('delete-annotations')"
        :disabled="!currentImage?.has_annotation"
      >
        删除标注 (JSON)
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
  overflow: hidden;
  z-index: 10;
  box-sizing: border-box;
}

.section {
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.section.scrollable {
  flex: 1;
  min-height: 0;
}

.single-toggle {
  width: 100%;
  margin-bottom: 5px;
  display: block;
}

.mode-btn {
  padding: 8px 0;
  border: 1px solid #0d6efd;
  background-color: white;
  color: #0d6efd;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  transition: all 0.3s ease;
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

.search-input {
  margin-bottom: 8px;
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

.image-list, .polygon-list {
  overflow-y: auto;
  flex: 1;
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
  width: 80px;
  padding: 4px;
  border: 1px solid #ddd;
  border-radius: 3px;
}

.inline-select {
  width: 24px;
  padding: 4px 0;
  color: transparent;
  border: 1px solid #ddd;
  border-radius: 3px;
  background-color: white;
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
  background-repeat: no-repeat;
  background-position: center;
  background-size: 14px;
}

.inline-select option {
  color: #000;
}

.inline-select:focus {
  outline: none;
  border-color: #0d6efd;
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
  display: flex;
  flex-direction: column;
  gap: 10px;
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

.delete-json-btn {
  width: 100%;
  padding: 12px;
  background-color: #dc3545;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  font-weight: bold;
}

.delete-json-btn:hover {
  background-color: #c82333;
}

.delete-json-btn:disabled {
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
