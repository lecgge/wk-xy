class ThreeStateSwitch extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({mode: 'open'});

        // 获取属性
        this.label = this.getAttribute('label') || '按钮';
        this.value = this.getAttribute('value') || 'off'; // off, on, auto

        // 创建结构
        const container = document.createElement('div');
        container.className = 'switch-container';

        const label = document.createElement('div');
        label.className = 'switch-label';
        label.textContent = this.label;

        const switchWrapper = document.createElement('div');
        switchWrapper.className = `toggle-switch ${this.value}`;
        switchWrapper.innerHTML = `
        <div class="toggle-option" data-value="off">Off</div>
        <div class="toggle-option" data-value="on">ACC</div>
        <div class="toggle-option" data-value="auto">RUN</div>
        <div class="slider"></div>
      `;

        // 样式注入
        const style = document.createElement('style');
        style.textContent = `
        :host {
          display: block;
          font-family: Arial, sans-serif;
          margin: 20px 0;
        }

        .switch-label {
          font-size: 16px;
          color: #333;
          margin-bottom: 10px;
        }

        .toggle-switch {
          position: relative;
          display: flex;
          width: 180px;
          height: 36px;
          background-color: white;
          border-radius: 18px;
          overflow: hidden;
          box-shadow: 0 2px 6px rgba(0,0,0,0.1);
          cursor: pointer;
          user-select: none;
        }

        .toggle-option {
          flex: 1;
          display: flex;
          justify-content: center;
          align-items: center;
          z-index: 1;
          position: relative;
          font-size: 14px;
          color: #555;
        }

        .slider {
          position: absolute;
          top: 4px;
          left: 4px;
          height: 28px;
          width: 56px;
          background-color: #007BFF;
          border-radius: 14px;
          transition: transform 0.3s ease;
          z-index: 0;
        }

        .toggle-switch.off .slider {
          transform: translateX(0);
        }

        .toggle-switch.on .slider {
          transform: translateX(60px);
        }

        .toggle-switch.auto .slider {
          transform: translateX(120px);
        }

        .toggle-switch .toggle-option:nth-child(1),
        .toggle-switch .toggle-option:nth-child(2),
        .toggle-switch .toggle-option:nth-child(3) {
          color: #555;
          font-weight: normal;
        }

        .toggle-switch.off .toggle-option:nth-child(1),
        .toggle-switch.on .toggle-option:nth-child(2),
        .toggle-switch.auto .toggle-option:nth-child(3) {
          color: white;
          font-weight: bold;
        }
      `;

        // 绑定点击事件
        switchWrapper.addEventListener('click', (e) => {
            const target = e.target.closest('.toggle-option');
            if (!target) return;

            const newValue = target.getAttribute('data-value');
            this.setAttribute('value', newValue); // 更新状态
            this.dispatchEvent(new CustomEvent('change', {
                detail: {value: newValue},
                bubbles: true,
                composed: true
            }));
        });

        // 初始化状态
        this.switchWrapper = switchWrapper;
        this.updateState(this.value);

        // 添加到 shadow DOM
        container.appendChild(label);
        container.appendChild(switchWrapper);
        this.shadowRoot.appendChild(style);
        this.shadowRoot.appendChild(container);
    }

    static get observedAttributes() {
        return ['value'];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === 'value') {
            this.updateState(newValue);
        }
    }

    updateState(value) {
        this.switchWrapper.classList.remove('off', 'on', 'auto');
        this.switchWrapper.classList.add(value);
    }
}

customElements.define('three-state-switch', ThreeStateSwitch);