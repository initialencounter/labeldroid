<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { ElMessage } from 'element-plus';
import Sidebar from './components/Sidebar.vue';
import CanvasArea from './components/CanvasArea.vue';
import type { ImageInfo, Shape, LabelmeData } from './types';
import { apiManager } from './utils/api';

const images = ref<ImageInfo[]>([]);
const currentImage = ref<ImageInfo | null>(null);

const shapes = ref<Shape[]>([]);
const currentPolygon = ref<{ x: number; y: number }[]>([]);
const mousePos = ref<{ x: number; y: number } | null>(null);
const selectedShapeIndex = ref<number | null>(null);
const defaultLabelName = ref<string>('BTY');

let htmlImage = new Image();
htmlImage.crossOrigin = 'anonymous';
const hasImageContent = ref(false);
let imgWidth = 0;
let imgHeight = 0;

let canvasEl: HTMLCanvasElement | null = null;
const canvasAreaRef = ref<InstanceType<typeof CanvasArea> | null>(null);

// ======== 模式切换 ========
const isEditMode = ref(false);
const draggingPoint = ref<{ shapeIndex: number; pointIndex: number } | null>(
  null,
);

watch(isEditMode, (newVal) => {
  if (newVal && currentPolygon.value.length > 0) {
    cancelCurrentPolygon();
  }
});

// 缩放和平移状态
const scale = ref(1);
const offset = ref({ x: 0, y: 0 });

// 交互状态
const isDragging = ref(false);
const isZoomingOrPinching = ref(false);
const lastPointerPos = ref({ x: 0, y: 0 });
const initialDistance = ref<number | null>(null);
const initialScale = ref<number>(1);
const activePointers = ref<Map<number, { x: number; y: number }>>(new Map());

const handleCanvasReady = (canvas: HTMLCanvasElement) => {
  canvasEl = canvas;
};

const fetchImages = async () => {
  try {
    const data = await apiManager.get('/api/images');
    images.value = data;
    if (data.length > 0 && !currentImage.value) {
      selectImage(data[0]);
    }
  } catch (error) {
    console.error('Failed to fetch images list', error);
  }
};

const selectShape = (index: number) => {
  selectedShapeIndex.value = index;
};

const deleteShape = (index: number) => {
  shapes.value.splice(index, 1);
  if (selectedShapeIndex.value === index) {
    selectedShapeIndex.value = null;
  }
};

const getCanvasPos = (clientX: number, clientY: number) => {
  const canvas = canvasEl;
  if (!canvas) return { x: 0, y: 0 };
  const rect = canvas.getBoundingClientRect();

  const basicX = (clientX - rect.left) * (canvas.width / rect.width);
  const basicY = (clientY - rect.top) * (canvas.height / rect.height);

  return {
    x: (basicX - offset.value.x) / scale.value,
    y: (basicY - offset.value.y) / scale.value,
  };
};

const getDistance = (
  p1: { x: number; y: number },
  p2: { x: number; y: number },
) => {
  return Math.sqrt(Math.pow(p2.x - p1.x, 2) + Math.pow(p2.y - p1.y, 2));
};

const getCenter = (
  p1: { x: number; y: number },
  p2: { x: number; y: number },
) => {
  return { x: (p1.x + p2.x) / 2, y: (p1.y + p2.y) / 2 };
};

const handlePointerDown = (e: PointerEvent | MouseEvent | TouchEvent) => {
  if (e instanceof MouseEvent && e.button === 2) {
    e.preventDefault();
    if (!isEditMode.value) finishCurrentPolygon();
    return;
  }
  if (!currentImage.value || !canvasEl) return;

  if (window.PointerEvent && e instanceof PointerEvent) {
    activePointers.value.set(e.pointerId, { x: e.clientX, y: e.clientY });

    if (activePointers.value.size === 1) {
      if (e.button === 1 || e.shiftKey) {
        isDragging.value = true;
        lastPointerPos.value = { x: e.clientX, y: e.clientY };
        return;
      }

      if (isZoomingOrPinching.value) {
        return;
      }

      const pos = getCanvasPos(e.clientX, e.clientY);

      if (isEditMode.value) {
        // 寻找最近的点
        const threshold = 15 / scale.value; // 点击容差，按缩放比例动态调整视觉容差
        let minDist = Infinity;
        let sIdx = -1;
        let pIdx = -1;

        shapes.value.forEach((shape, i) => {
          shape.points.forEach((point, j) => {
            const dist = getDistance(pos, { x: point[0], y: point[1] });
            if (dist < minDist && dist <= threshold) {
              minDist = dist;
              sIdx = i;
              pIdx = j;
            }
          });
        });

        if (sIdx !== -1 && pIdx !== -1) {
          draggingPoint.value = { shapeIndex: sIdx, pointIndex: pIdx };
          selectedShapeIndex.value = sIdx;
        } else {
          draggingPoint.value = null; // 点击空白处
        }
      } else {
        currentPolygon.value.push(pos);
      }
    } else if (activePointers.value.size === 2) {
      isZoomingOrPinching.value = true;
      isDragging.value = false;
      draggingPoint.value = null;

      if (!isEditMode.value && currentPolygon.value.length > 0) {
        currentPolygon.value.pop();
      }

      const points = Array.from(activePointers.value.values());
      initialDistance.value = getDistance(points[0], points[1]);
      initialScale.value = scale.value;
    }
  }
};

const handlePointerMove = (e: PointerEvent | MouseEvent | TouchEvent) => {
  if (!currentImage.value || !canvasEl) return;

  if (window.PointerEvent && e instanceof PointerEvent) {
    if (activePointers.value.has(e.pointerId)) {
      activePointers.value.set(e.pointerId, { x: e.clientX, y: e.clientY });
    }

    if (activePointers.value.size === 1) {
      if (isDragging.value) {
        // 拖拽平移画布
        const dx =
          (e.clientX - lastPointerPos.value.x) *
          (canvasEl.width / canvasEl.clientWidth);
        const dy =
          (e.clientY - lastPointerPos.value.y) *
          (canvasEl.height / canvasEl.clientHeight);

        offset.value.x += dx;
        offset.value.y += dy;

        lastPointerPos.value = { x: e.clientX, y: e.clientY };
        return;
      }

      if (isEditMode.value && draggingPoint.value !== null) {
        // 编辑点
        // 移动设备下，将点向上偏移，避免手指遮挡
        const offsetY = e.pointerType === 'touch' ? -50 : 0;
        const offsetX = e.pointerType === 'touch' ? -50 : 0;
        const pos = getCanvasPos(e.clientX + offsetX, e.clientY + offsetY);
        const { shapeIndex, pointIndex } = draggingPoint.value;
        shapes.value[shapeIndex].points[pointIndex] = [
          Math.round(pos.x),
          Math.round(pos.y),
        ];
        return;
      }
    }

    if (activePointers.value.size === 2 && initialDistance.value !== null) {
      const points = Array.from(activePointers.value.values());
      const newDistance = getDistance(points[0], points[1]);

      const scaleRatio = newDistance / initialDistance.value;
      let newScale = initialScale.value * scaleRatio;

      newScale = Math.max(0.1, Math.min(newScale, 10));

      const center = getCenter(points[0], points[1]);
      const rect = canvasEl.getBoundingClientRect();

      const canvasRelX = (center.x - rect.left) * (canvasEl.width / rect.width);
      const canvasRelY =
        (center.y - rect.top) * (canvasEl.height / rect.height);

      offset.value.x =
        canvasRelX - (canvasRelX - offset.value.x) * (newScale / scale.value);
      offset.value.y =
        canvasRelY - (canvasRelY - offset.value.y) * (newScale / scale.value);
      scale.value = newScale;
      return;
    }

    if (
      activePointers.value.size <= 1 &&
      !isDragging.value &&
      !isZoomingOrPinching.value
    ) {
      mousePos.value = getCanvasPos(e.clientX, e.clientY);
    } else {
      mousePos.value = null; // 隐藏准星
    }
  }
};

const handlePointerUp = (e: PointerEvent | MouseEvent | TouchEvent) => {
  if (window.PointerEvent && e instanceof PointerEvent) {
    activePointers.value.delete(e.pointerId);

    if (activePointers.value.size < 2) {
      initialDistance.value = null;
    }
    if (activePointers.value.size === 0) {
      isDragging.value = false;
      draggingPoint.value = null;
      setTimeout(() => {
        isZoomingOrPinching.value = false;
      }, 50);
    }
  }
};

const handleWheel = (e: WheelEvent) => {
  if (!currentImage.value || !canvasEl) return;

  const zoomFactor = 1.1;
  const direction = e.deltaY < 0 ? 1 : -1;
  let newScale = scale.value * (direction > 0 ? zoomFactor : 1 / zoomFactor);

  newScale = Math.max(0.1, Math.min(newScale, 10));

  const rect = canvasEl.getBoundingClientRect();
  const canvasRelX = (e.clientX - rect.left) * (canvasEl.width / rect.width);
  const canvasRelY = (e.clientY - rect.top) * (canvasEl.height / rect.height);

  offset.value.x =
    canvasRelX - (canvasRelX - offset.value.x) * (newScale / scale.value);
  offset.value.y =
    canvasRelY - (canvasRelY - offset.value.y) * (newScale / scale.value);

  scale.value = newScale;
};

const finishCurrentPolygon = () => {
  if (currentPolygon.value.length >= 3) {
    shapes.value.push({
      label: defaultLabelName.value || 'polygon',
      points: currentPolygon.value.map((p: { x: number; y: number }) => [
        p.x,
        p.y,
      ]),
      group_id: null,
      shape_type: 'polygon',
      flags: {},
    });
    selectedShapeIndex.value = shapes.value.length - 1;
  }
  currentPolygon.value = [];
  mousePos.value = null;
};

const cancelCurrentPolygon = () => {
  currentPolygon.value = [];
  mousePos.value = null;
};

const handleRightClick = (e: Event) => {
  e.preventDefault();
  if (!isEditMode.value) {
    finishCurrentPolygon();
  }
};

const fitImageToCanvas = () => {
  if (!canvasEl || !htmlImage.complete) return;

  const container = canvasEl.parentElement;
  if (container) {
    canvasEl.width = container.clientWidth;
    canvasEl.height = container.clientHeight;
  }

  const scaleX = canvasEl.width / htmlImage.width;
  const scaleY = canvasEl.height / htmlImage.height;
  let bestScale = Math.min(scaleX, scaleY) * 0.9;

  if (isNaN(bestScale) || !isFinite(bestScale)) bestScale = 1;

  scale.value = bestScale;

  offset.value = {
    x: (canvasEl.width - htmlImage.width * bestScale) / 2,
    y: (canvasEl.height - htmlImage.height * bestScale) / 2,
  };
};

const selectImage = async (img: ImageInfo) => {
  if (currentImage.value && currentImage.value.name !== img.name) {
    await saveAnnotationsSilently();
  }

  currentImage.value = img;
  shapes.value = [];
  currentPolygon.value = [];
  selectedShapeIndex.value = null;
  hasImageContent.value = false;
  scale.value = 1;
  offset.value = { x: 0, y: 0 };

  htmlImage.onload = async () => {
    imgWidth = htmlImage.width;
    imgHeight = htmlImage.height;

    fitImageToCanvas();
    hasImageContent.value = true;

    if (img.has_annotation) {
      try {
        const res: LabelmeData = await apiManager.get(
          `/api/annotations/${img.name}`,
        );
        shapes.value = res.shapes || [];
      } catch (err) {
        console.error('No valid annotation data');
      }
    }
  };

  htmlImage.src = `http://127.0.0.1:${apiManager.serverPort}/api/images/${img.name}?t=${Date.now()}`;
};

const saveAnnotationsSilently = async () => {
  if (!currentImage.value) return;

  const payload: LabelmeData = {
    version: '5.4.1',
    flags: {},
    shapes: shapes.value,
    imagePath: currentImage.value.name,
    imageData: null,
    imageHeight: imgHeight,
    imageWidth: imgWidth,
  };

  if (shapes.value.length === 0) {
    console.info('No shapes to save, skipping annotation save.');
    return;
  }

  try {
    const res = await apiManager.post(
      `/api/annotations/${currentImage.value.name}`,
      payload,
    );
    console.info('Auto-saving annotation for image:', res);
    if (res.ok) {
      await fetchImages();
    }
  } catch (error) {
    console.error('Auto save failed:', error);
  }
};

const saveAnnotations = async () => {
  if (!currentImage.value) return;

  const payload: LabelmeData = {
    version: '5.4.1',
    flags: {},
    shapes: shapes.value,
    imagePath: currentImage.value.name,
    imageData: null,
    imageHeight: imgHeight,
    imageWidth: imgWidth,
  };

  if (shapes.value.length === 0) {
    console.info('No shapes to save, skipping annotation save.');
    return;
  }

  try {
    const res = await apiManager.post(
      `/api/annotations/${currentImage.value.name}`,
      payload,
    );
    console.info('Save response:', res);
    if (res.ok) {
      ElMessage.success('保存成功!');
      await fetchImages();
    } else {
      ElMessage.error('保存失败');
    }
  } catch (error) {
    console.error(error);
    ElMessage.error('保存错误');
  }
};

onMounted(() => {
  fetchImages();
  window.addEventListener('resize', () => {
    if (hasImageContent.value) {
      fitImageToCanvas();
      canvasAreaRef.value?.draw();
    }
  });
});
</script>

<template>
  <div class="labelme-container">
    <Sidebar
      :images="images"
      :currentImage="currentImage"
      :shapes="shapes"
      :selectedShapeIndex="selectedShapeIndex"
      :defaultLabelName="defaultLabelName"
      :canSave="!!currentImage"
      :isEditMode="isEditMode"
      @update:isEditMode="isEditMode = $event"
      @select-image="selectImage"
      @refresh-images="fetchImages"
      @update:defaultLabelName="defaultLabelName = $event"
      @select-shape="selectShape"
      @delete-shape="deleteShape"
      @save-annotations="saveAnnotations"
      @shape-label-change="canvasAreaRef?.draw()"
    />

    <CanvasArea
      ref="canvasAreaRef"
      :isEditMode="isEditMode"
      :shapes="shapes"
      :currentPolygon="currentPolygon"
      :mousePos="mousePos"
      :selectedShapeIndex="selectedShapeIndex"
      :htmlImage="htmlImage"
      :hasImageContent="hasImageContent"
      :scale="scale"
      :offset="offset"
      @canvas-ready="handleCanvasReady"
      @pointer-down="handlePointerDown"
      @pointer-move="handlePointerMove"
      @pointer-up="handlePointerUp"
      @wheel="handleWheel"
      @right-click="handleRightClick"
      @finish-polygon="finishCurrentPolygon"
      @cancel-polygon="cancelCurrentPolygon"
    />
  </div>
</template>

<style scoped>
.labelme-container {
  display: flex;
  height: 100vh;
  width: 100vw;
  font-family: Arial, sans-serif;
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  overflow: hidden;
}

@media (max-width: 768px) {
  .labelme-container {
    flex-direction: column;
  }
}
</style>
