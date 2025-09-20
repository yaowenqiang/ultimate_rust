# Footgun Manual - Makefile
# 简化项目构建和运行的便捷工具

.PHONY: help run run-release optimized optimized-release compare bench clean check fmt clippy test all install-deps

# 默认目标：显示帮助信息
help:
	@echo "🔧 Footgun Manual - 数据竞争修复练习"
	@echo ""
	@echo "📋 可用命令:"
	@echo "  make run              - 运行你的修复版本 (调试模式)"
	@echo "  make run-release      - 运行你的修复版本 (发布模式)"
	@echo "  make optimized        - 运行优化版本 (调试模式)"
	@echo "  make optimized-release- 运行优化版本 (发布模式)"
	@echo "  make compare          - 对比三个版本的输出"
	@echo "  make bench            - 性能基准测试"
	@echo "  make check            - 检查代码编译"
	@echo "  make clippy           - 运行 clippy 代码检查"
	@echo "  make fmt              - 格式化代码"
	@echo "  make test             - 运行测试"
	@echo "  make clean            - 清理构建文件"
	@echo "  make all              - 运行所有检查和测试"
	@echo "  make install-deps     - 安装依赖工具"
	@echo ""
	@echo "🎯 快速开始: make run"

# 运行你的修复版本
run:
	@echo "🚀 运行你的修复版本..."
	cargo run

# 运行你的修复版本 (发布模式)
run-release:
	@echo "🚀 运行你的修复版本 (发布模式)..."
	cargo run --release

# 编译并运行优化版本
optimized:
	@echo "🔧 编译优化版本..."
	rustc src/main_optimized.rs -o main_optimized
	@echo "🚀 运行优化版本..."
	./main_optimized
	@echo "🧹 清理可执行文件..."
	rm -f main_optimized

# 编译并运行优化版本 (发布模式)
optimized-release:
	@echo "🔧 编译优化版本 (发布模式)..."
	rustc src/main_optimized.rs -O -o main_optimized
	@echo "🚀 运行优化版本..."
	./main_optimized
	@echo "🧹 清理可执行文件..."
	rm -f main_optimized

# 对比三个版本的输出
compare:
	@echo "📊 对比所有版本的运行结果"
	@echo ""
	@echo "=== 1. 原始版本 (有数据竞争) ==="
	@cd ../footgun && cargo run --release 2>/dev/null | head -20
	@echo ""
	@echo "=== 2. 你的修复版本 ==="
	@cargo run --release 2>/dev/null
	@echo ""
	@echo "=== 3. 优化版本 ==="
	@rustc src/main_optimized.rs -O -o main_optimized 2>/dev/null
	@./main_optimized | head -25
	@rm -f main_optimized
	@echo ""
	@echo "✅ 对比完成！注意数据竞争版本的结果每次都不同"

# 性能基准测试
bench:
	@echo "⏱️  性能基准测试 (5次运行)"
	@echo ""
	@echo "--- 你的版本 (发布模式) ---"
	@for i in 1 2 3 4 5; do \
		echo -n "Run $$i: "; \
		time cargo run --release 2>&1 | grep -E "(real|user|sys)" | tr -d '\n'; \
		echo ""; \
	done
	@echo ""
	@echo "--- 优化版本 (发布模式) ---"
	@rustc src/main_optimized.rs -O -o main_optimized 2>/dev/null
	@for i in 1 2 3 4 5; do \
		echo -n "Run $$i: "; \
		time ./main_optimized 2>&1 | grep -E "(real|user|sys)" | tr -d '\n'; \
		echo ""; \
	done
	@rm -f main_optimized

# 检查代码编译
check:
	@echo "🔍 检查代码编译..."
	cargo check
	@echo "🔍 检查优化版本..."
	rustc --check src/main_optimized.rs

# 运行 clippy 代码检查
clippy:
	@echo "📎 运行 clippy 检查..."
	cargo clippy -- -D warnings
	@echo "📎 检查优化版本..."
	rustc src/main_optimized.rs --cfg clippy

# 格式化代码
fmt:
	@echo "🎨 格式化代码..."
	cargo fmt
	@echo "🎨 格式化优化版本..."
	rustfmt src/main_optimized.rs

# 运行测试
test:
	@echo "🧪 运行测试..."
	cargo test
	@echo "🧪 验证优化版本输出..."
	@rustc src/main_optimized.rs -O -o main_optimized 2>/dev/null
	@result=$$(./main_optimized | grep "最终计数" | grep -o "[0-9]*"); \
	if [ "$$result" = "1000000" ]; then \
		echo "✅ 优化版本测试通过"; \
	else \
		echo "❌ 优化版本测试失败: $$result"; \
	fi
	@rm -f main_optimized

# 清理构建文件
clean:
	@echo "🧹 清理构建文件..."
	cargo clean
	rm -f main_optimized
	rm -f *.pdb
	@echo "✅ 清理完成"

# 运行所有检查和测试
all: check clippy fmt test
	@echo "✅ 所有检查通过！"

# 安装依赖工具
install-deps:
	@echo "📦 安装开发依赖..."
	rustup component add clippy
	rustup component add rustfmt
	@echo "✅ 依赖安装完成"

# 快速验证修复是否成功
validate:
	@echo "🔬 验证数据竞争修复..."
	@echo ""
	@echo "测试你的版本 (5次运行):"
	@consistent=true; \
	for i in 1 2 3 4 5; do \
		result=$$(cargo run --release 2>/dev/null); \
		if [ "$$result" != "1000000" ]; then \
			echo "Run $$i: ❌ $$result (不正确)"; \
			consistent=false; \
		else \
			echo "Run $$i: ✅ $$result (正确)"; \
		fi; \
	done; \
	if [ "$$consistent" = "true" ]; then \
		echo "🎉 修复验证通过！结果完全一致且正确"; \
	else \
		echo "⚠️  修复可能有问题，结果不一致"; \
	fi

# 显示项目信息
info:
	@echo "📁 项目信息:"
	@echo "  当前目录: $$(pwd)"
	@echo "  Rust版本: $$(rustc --version)"
	@echo "  Cargo版本: $$(cargo --version)"
	@echo ""
	@echo "📂 文件结构:"
	@find . -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "Makefile" | sort

# 生成文档
docs:
	@echo "📖 生成文档..."
	cargo doc --open

# 统计代码行数
stats:
	@echo "📊 代码统计:"
	@echo "你的版本 (main.rs):"
	@wc -l src/main.rs
	@echo "优化版本 (main_optimized.rs):"
	@wc -l src/main_optimized.rs
	@echo "总计:"
	@find src -name "*.rs" -exec wc -l {} + | tail -1