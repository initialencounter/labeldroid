<script setup lang="ts">
import { ref, computed, onMounted, watch, ComputedRef } from 'vue';
import type { Shape } from '../types';

const props = defineProps<{
  shapes: Shape[];
  currentPolygon: { x: number; y: number }[];
  mousePos: { x: number; y: number } | null;
  selectedShapeIndex: number | null;
  htmlImage: HTMLImageElement;
  hasImageContent: boolean;
  scale: number;
  offset: { x: number; y: number };
  isEditMode: boolean;
}>();

const emit = defineEmits<{
  (e: 'pointer-down', event: PointerEvent | MouseEvent | TouchEvent): void;
  (e: 'pointer-move', event: PointerEvent | MouseEvent | TouchEvent): void;
  (e: 'pointer-up', event: PointerEvent | MouseEvent | TouchEvent): void;
  (e: 'wheel', event: WheelEvent): void;
  (e: 'right-click', event: Event): void;
  (e: 'finish-polygon'): void;
  (e: 'cancel-polygon'): void;
  (e: 'canvas-ready', canvas: HTMLCanvasElement): void;
}>();

const canvasRef = ref<HTMLCanvasElement | null>(null);

// 可自定义的多边形线段和点的尺寸变量
const STROKE_WIDTH = 1;
const STROKE_WIDTH_SELECTED = 2;
const POINT_RADIUS = 1.5;
const POINT_RADIUS_SELECTED = 2.5;

const hoveredPoint: ComputedRef<{
  shapeIndex: number;
  pointIndex: number;
} | null> = computed(() => {
  if (!props.isEditMode || !props.mousePos) return null;
  const pos = props.mousePos;
  const threshold = 15 / props.scale;
  let minDist = Infinity;
  let result: { shapeIndex: number; pointIndex: number } | null = null;

  props.shapes.forEach((shape, i) => {
    shape.points.forEach((point, j) => {
      const dist = Math.sqrt((pos.x - point[0]) ** 2 + (pos.y - point[1]) ** 2);
      if (dist < minDist && dist <= threshold) {
        minDist = dist;
        result = { shapeIndex: i, pointIndex: j };
      }
    });
  });

  return result;
});

const cursorStyle = computed(() => {
  if (props.isEditMode) {
    return hoveredPoint.value ? 'crosshair' : 'default';
  }
  return 'crosshair';
});

onMounted(() => {
  if (canvasRef.value) {
    emit('canvas-ready', canvasRef.value);

    // 监听键盘快捷键放大缩小
    window.addEventListener('keydown', handleKeyDown);
  }
});

const handleKeyDown = (e: KeyboardEvent) => {
  // Ctrl + 和 Ctrl - 缩放
  if (e.ctrlKey || e.metaKey) {
    if (e.key === '=' || e.key === '+') {
      e.preventDefault();
      emit('wheel', {
        deltaY: -100,
        preventDefault: () => {},
        clientX: window.innerWidth / 2,
        clientY: window.innerHeight / 2,
      } as any);
    } else if (e.key === '-') {
      e.preventDefault();
      emit('wheel', {
        deltaY: 100,
        preventDefault: () => {},
        clientX: window.innerWidth / 2,
        clientY: window.innerHeight / 2,
      } as any);
    }
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

const getColorByLabel = (label: string) => {
  // 获取当前所有的唯一标签并按顺序去重
  const uniqueLabels = Array.from(new Set(props.shapes.map((s) => s.label)));
  // 查找当前 label 所在的索引
  let index = uniqueLabels.indexOf(label);
  if (index === -1) {
    index = uniqueLabels.length; // 新标签预判
  }
  // labelme 中 label_id 默认从 1 开始（跳过黑色）
  const labelId = (1 + index) % vocColormap.length;
  return vocColormap[labelId];
};

const draw = () => {
  const canvas = canvasRef.value;
  if (!canvas) return;
  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  // 清除整个画布（使用视觉尺寸）
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  // 保存当前状态
  ctx.save();

  // 应用缩放和平移变换
  ctx.translate(props.offset.x, props.offset.y);
  ctx.scale(props.scale, props.scale);

  if (props.hasImageContent && props.htmlImage.complete) {
    // 图片本身的宽高就是其实际像素
    ctx.drawImage(
      props.htmlImage,
      0,
      0,
      props.htmlImage.width,
      props.htmlImage.height,
    );
  }

  props.shapes.forEach((shape: Shape, index: number) => {
    const points = shape.points.map((p) => ({ x: p[0], y: p[1] }));
    const baseColor = getColorByLabel(shape.label);
    const isSelected = props.selectedShapeIndex === index;

    // HSL 转换为 RGBA 以调整透明度
    const fillAlpha = isSelected ? 0.6 : 0.2;
    // 使用临时 canvas 或者 ctx.globalAlpha 来实现颜色带透明度
    // 为了简单，我们只改变颜色模式，直接画
    const hoverIndex =
      hoveredPoint.value?.shapeIndex === index
        ? hoveredPoint.value.pointIndex
        : -1;
    drawPolygonWithLayer(
      ctx,
      points,
      baseColor,
      fillAlpha,
      isSelected,
      hoverIndex,
    );
  });

  if (props.currentPolygon.length > 0) {
    drawPolygon(
      ctx,
      props.currentPolygon,
      'rgba(255, 0, 0, 0.3)',
      'rgba(255, 0, 0, 1)',
      false,
      false,
      -1,
    );

    if (props.mousePos) {
      ctx.beginPath();
      const lastPoint = props.currentPolygon[props.currentPolygon.length - 1];
      ctx.moveTo(lastPoint.x, lastPoint.y);
      ctx.lineTo(props.mousePos.x, props.mousePos.y);
      ctx.strokeStyle = 'red';
      ctx.lineWidth = STROKE_WIDTH / props.scale; // 保持线条视觉宽度一致
      ctx.setLineDash([5 / props.scale, 5 / props.scale]);
      ctx.stroke();
      ctx.setLineDash([]);
    }
  }

  ctx.restore();
};

const drawPolygonWithLayer = (
  ctx: CanvasRenderingContext2D,
  points: { x: number; y: number }[],
  color: string,
  alpha: number,
  isSelected: boolean,
  hoverIndex: number,
) => {
  if (points.length === 0) return;

  ctx.beginPath();
  ctx.moveTo(points[0].x, points[0].y);
  for (let i = 1; i < points.length; i++) {
    ctx.lineTo(points[i].x, points[i].y);
  }
  if (points.length > 2) ctx.closePath();

  // 解析并设置透明度
  const rgbMatch = color.match(/rgb\((\d+),\s*(\d+),\s*(\d+)\)/);
  if (rgbMatch) {
    ctx.fillStyle = `rgba(${rgbMatch[1]}, ${rgbMatch[2]}, ${rgbMatch[3]}, ${alpha})`;
  } else {
    ctx.fillStyle = `rgba(200, 200, 200, ${alpha})`;
  }

  ctx.fill();
  ctx.strokeStyle = isSelected ? '#FFFFFF' : color;
  ctx.lineWidth =
    (isSelected ? STROKE_WIDTH_SELECTED : STROKE_WIDTH) / props.scale;
  ctx.stroke();

  points.forEach((p, idx) => {
    ctx.beginPath();
    const isHovered = idx === hoverIndex && props.isEditMode;
    const radius = isHovered
      ? POINT_RADIUS_SELECTED + 2
      : isSelected
        ? POINT_RADIUS_SELECTED
        : POINT_RADIUS;
    ctx.arc(p.x, p.y, radius / props.scale, 0, Math.PI * 2);
    ctx.fillStyle = isHovered ? '#00FF00' : ctx.strokeStyle;
    ctx.fill();
    if (isHovered) {
      ctx.strokeStyle = '#FFFFFF';
      ctx.lineWidth = 1 / props.scale;
      ctx.stroke();
      ctx.strokeStyle = isSelected ? '#FFFFFF' : color; // 恢复画笔颜色
    }
  });
};

const drawPolygon = (
  ctx: CanvasRenderingContext2D,
  points: { x: number; y: number }[],
  fillColor: string,
  strokeColor: string,
  isClosed: boolean,
  isSelected: boolean,
  hoverIndex: number,
) => {
  if (points.length === 0) return;

  ctx.beginPath();
  ctx.moveTo(points[0].x, points[0].y);
  for (let i = 1; i < points.length; i++) {
    ctx.lineTo(points[i].x, points[i].y);
  }

  if (isClosed && points.length > 2) {
    ctx.closePath();
  }

  ctx.fillStyle = fillColor;
  ctx.fill();
  ctx.strokeStyle = strokeColor;
  ctx.lineWidth =
    (isSelected ? STROKE_WIDTH_SELECTED : STROKE_WIDTH) / props.scale;
  ctx.stroke();

  points.forEach((p, idx) => {
    ctx.beginPath();
    const isHovered = idx === hoverIndex && props.isEditMode;
    const radius = isHovered
      ? POINT_RADIUS_SELECTED + 2
      : isSelected
        ? POINT_RADIUS_SELECTED
        : POINT_RADIUS;
    ctx.arc(p.x, p.y, radius / props.scale, 0, Math.PI * 2);
    ctx.fillStyle = isHovered ? '#00FF00' : strokeColor;
    ctx.fill();
    if (isHovered) {
      ctx.strokeStyle = '#FFFFFF';
      ctx.lineWidth = 1 / props.scale;
      ctx.stroke();
      ctx.strokeStyle = strokeColor;
    }
  });
};

defineExpose({
  draw,
  getColorByLabel,
});

watch(
  [
    () => props.shapes,
    () => props.currentPolygon,
    () => props.mousePos,
    () => props.selectedShapeIndex,
    () => props.hasImageContent,
    () => props.scale,
    () => props.offset,
  ],
  () => {
    draw();
  },
  { deep: true },
);
</script>

<template>
  <div class="main-content">
    <div class="toolbar" v-if="currentPolygon.length > 0">
      <button class="tool-btn finish" @click="emit('finish-polygon')">
        ✔️ 闭合 (或右击)
      </button>
      <button class="tool-btn cancel" @click="emit('cancel-polygon')">
        ❌ 终止
      </button>
    </div>

    <div class="canvas-wrapper">
      <canvas
        ref="canvasRef"
        @pointerdown="emit('pointer-down', $event)"
        @pointermove="emit('pointer-move', $event)"
        @pointerup="emit('pointer-up', $event)"
        @pointercancel="emit('pointer-up', $event)"
        @pointerleave="emit('pointer-up', $event)"
        @wheel.prevent="emit('wheel', $event)"
        @contextmenu="emit('right-click', $event)"
        class="drawing-canvas"
        :style="{ cursor: cursorStyle }"
      ></canvas>
    </div>
    <div class="instructions">
      <p>
        <strong>操作：</strong
        >左键/点击画点；双指捏合/滚轮缩放；按住拖动画布；右击闭合；侧栏改名。
      </p>
    </div>
  </div>
</template>

<style scoped>
.main-content {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-color: #e9ecef;
  padding: 20px;
  position: relative;
  overflow: hidden; /* 防止撑开导致滚动条 */
}

.toolbar {
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 10px;
  background: rgba(255, 255, 255, 0.9);
  padding: 10px 20px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 100;
}

.tool-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-weight: bold;
  cursor: pointer;
}

.tool-btn.finish {
  background-color: #0d6efd;
  color: white;
}
.tool-btn.cancel {
  background-color: #6c757d;
  color: white;
}

.canvas-wrapper {
  width: 100%;
  height: 100%;
  overflow: hidden;
  border: 1px solid #ccc;
  background: #333;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  touch-action: none;
  display: flex;
  justify-content: center;
  align-items: center;
}

.drawing-canvas {
  display: block;
  cursor: crosshair;
}

.instructions {
  margin-top: 15px;
  color: #6c757d;
  font-size: 14px;
  text-align: center;
  position: absolute;
  bottom: 10px;
  background: rgba(255, 255, 255, 0.8);
  padding: 5px 10px;
  border-radius: 4px;
}

@media (max-width: 768px) {
  .main-content {
    height: 65vh;
    padding: 0;
  }
  .toolbar {
    top: 5px;
    padding: 5px 10px;
  }
  .instructions {
    display: none; /* 移动端空间小，隐藏提示 */
  }
}
</style>
