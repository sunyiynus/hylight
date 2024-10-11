<template>
  <div class="container">
    <!-- 左侧控制面板 -->
    <div class="sidebar">
      <div class="control-group">
        <!-- <input type="file" class="styled-input" id="fileInput" accept="image/*" /> -->
        <input type="file" accept="image/*" @change="onFileChange" />
      </div>
      <div class="control-group">
        <label>changeBrightness</label>
        <input id="brightness" type="range" min="0" max="255" value="40" oninput="changeBrightness()" />
      </div>
      <div class="control-group">
        <button @click="toggleImage()" id="maskLayerButton">显示高亮图区</button>
        <button @click="handleButtonClick(6)">Button 6</button>
      </div>
    </div>

    <!-- 右侧内容显示区 -->
    <div class="content">
      <div v-if="processedImage">
        <img :src="originalImage" class="img1" alt="Original Image" />
        <img id="maskLayer" :src="processedImage" />
        <!-- <img :src="processedImage" alt="Original Image2" /> -->
        <!-- 底层 -->
        <!-- <canvas id="backgroundLayer" class="layer" width="500" height="500"></canvas> -->
        <!-- 中间层 -->
        <!-- <canvas id="middleLayer" class="layer" width="500" height="500"></canvas> -->
        <!-- 顶层 -->
        <!-- <canvas id="topLayer" class="layer" width="500" height="500"></canvas> --> -->
      </div>
    </div>
  </div>
</template>

<script>
import './assets/modern-normalize.css';
import { invoke } from "@tauri-apps/api/core";





export default {
  name: "App",
  data() {
    return {
      originalImage: null,
      processedImage: null,
    }
  },
  mounted() {
    // 在页面加载时，添加全局粘贴事件监听器
    window.addEventListener("paste", this.handlePaste);
  },
  beforeDestroy() {
    // 组件销毁时移除监听器，防止内存泄漏
    window.removeEventListener("paste", this.handlePaste);
  },
  methods: {
    handleButtonClick(buttonNumber) {
      console.log(`Button ${buttonNumber} clicked`);
      // 在这里处理按钮点击事件
    },
    onFileChange(event) {
      const file = event.target.files[0]
      if (file) {
        const reader = new FileReader()
        reader.onload = async (e) => {
          this.originalImage = e.target.result
          // 将图片数据发送到后端进行处理
          const result = await invoke('process_image', { imageData: this.originalImage })
          // const result = invoke('process_image', { imageData: this.originalImage })
          // console.log("Affter process", result)
          this.processedImage = result
        }
        reader.readAsDataURL(file)
      }
    },
    toggleImage() {
      const img = document.getElementById("maskLayer");
      const button = document.querySelector("#maskLayerButton");
      console.log(img);
      console.log(`Button maskLayerButton clicked`);

      // 检查图片是否隐藏
      if (img.style.display === "none") {
        img.style.display = "block";
        button.textContent = "隐藏图片图罩";
      } else {
        img.style.display = "none";
        button.textContent = "显示图片图罩";
      }
    },
    handlePaste(event) {
      // 防止默认粘贴行为
      event.preventDefault();
      console.log(`handlePaste`);

      const clipboardItems = event.clipboardData.items;

      for (const item of clipboardItems) {
        // 检查是否有图片类型的数据
        if (item.type.startsWith("image")) {
          const file = item.getAsFile();

          // 创建 FileReader 读取图片数据
          const reader = new FileReader();
          reader.onload = async (e) => {
            this.originalImage = e.target.result; // 读取图片数据并保存到 data 中
            const result = await invoke('process_image', { imageData: this.originalImage })
            this.processedImage = result

          };
          reader.readAsDataURL(file); // 将图片读取为 base64
        }
      }
    },
  },
};
</script>



<style>
/* * {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
} */

/* 根元素强制全屏无滚动 */
html,
body {
  margin: 0;
  padding: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  font-family: Arial, sans-serif;
}


/* 容器占据整个视口 */
.container {
  display: grid;
  grid-template-columns: 2fr 5fr;
  height: 100vh;
  width: 100vw;
  gap: 10px;
  padding: 10px;
  box-sizing: border-box;
}

/* 控制面板 */
.sidebar {
  display: flex;
  flex-direction: column;
  gap: 20px;
  border: 1px solid #000;
  padding: 10px;
  overflow-y: hidden;
  /* resize: horizontal; */
}

.control-group {
  display: flex;
  justify-content: space-between;
  gap: 10px;
}

.control-group button {
  flex: 1;
  padding: 10px;
  border: 1px solid #000;
  background-color: #f4f4f4;
  cursor: pointer;
}

/* 内容区 */
.content {
  border: 1px solid #000;
  padding: 10px;
  width: 100%;
  height: 100%;
  /* 也可以设置其他具体高度 */
  /* display: flex; */
  background-color: #f0f0f0;
  /* resize: horizontal; */
  overflow: hidden;
  position: relative;
}

.content img {
  /* 保持图片纵横比且适应容器 */
  position: absolute;
  align-items: center;
  justify-content: center;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.img1 {
  opacity: 1;
}

.img2 {
  display: none;
  opacity: 0.4;
}

#maskLayer {
  display: none;
  opacity: 0.4;
}
</style>
