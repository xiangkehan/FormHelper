#!/usr/bin/env python3
"""
进度管理工具 - FormHelper 项目
用于 AI Agent 读写进度状态，确保执行顺序正确

使用方法:
    python progress_manager.py read                    # 读取当前进度
    python progress_manager.py start <phase>           # 开始某阶段
    python progress_manager.py complete <phase>        # 完成某阶段
    python progress_manager.py task <phase> <task>     # 完成单个任务
    python progress_manager.py note "<message>"        # 添加备注
    python progress_manager.py check <phase>           # 检查依赖是否满足
"""

import json
import sys
from datetime import datetime
from pathlib import Path

PROGRESS_FILE = Path(__file__).parent / "progress.json"


def load_progress():
    """加载进度文件"""
    if not PROGRESS_FILE.exists():
        print("错误: 进度文件不存在")
        sys.exit(1)

    with open(PROGRESS_FILE, "r", encoding="utf-8") as f:
        return json.load(f)


def save_progress(data):
    """保存进度文件"""
    data["lastUpdated"] = datetime.now().isoformat()
    with open(PROGRESS_FILE, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)
    print("进度已保存")


def read_progress():
    """读取并显示当前进度"""
    data = load_progress()

    print("\n" + "=" * 60)
    print(f"FormHelper 项目进度")
    print(f"最后更新: {data['lastUpdated']}")
    print("=" * 60)

    for phase_key in data["executionOrder"]:
        phase = data["phases"].get(phase_key)
        if not phase:
            continue

        # 检查状态
        status_icon = {
            "pending": "⏳",
            "in_progress": "🔄",
            "completed": "✅",
            "blocked": "🚫"
        }.get(phase.get("status"), "❓")

        # 检查依赖
        deps = phase.get("dependencies", [])
        deps_met = all(
            data["phases"].get(d, {}).get("status") == "completed"
            for d in deps
        ) if deps else True

        dep_status = "✓ 依赖满足" if deps_met else "✗ 依赖未满足"

        print(f"\n{status_icon} {phase['name']}")
        print(f"   状态: {phase.get('status', 'unknown')}")
        print(f"   {dep_status}")

        # 显示并行 agent
        if phase.get("parallel"):
            for agent in phase.get("agents", []):
                print(f"   - {agent}")

        # 显示任务进度
        tasks = phase.get("tasks") or phase.get("agent3", {}).get("tasks") or phase.get("agent4", {}).get("tasks")
        if tasks:
            completed = sum(1 for t in tasks.values() if t.get("status") == "completed")
            total = len(tasks)
            print(f"   任务: {completed}/{total}")

    print("\n" + "=" * 60)
    return data


def start_phase(phase_key):
    """开始某阶段"""
    data = load_progress()

    phase = data["phases"].get(phase_key)
    if not phase:
        print(f"错误: 阶段 '{phase_key}' 不存在")
        sys.exit(1)

    # 检查依赖
    deps = phase.get("dependencies", [])
    for dep in deps:
        dep_phase = data["phases"].get(dep)
        if dep_phase and dep_phase.get("status") != "completed":
            print(f"错误: 依赖 {dep} 尚未完成，无法开始 {phase_key}")
            sys.exit(1)

    phase["status"] = "in_progress"
    phase["startTime"] = datetime.now().isoformat()
    save_progress(data)
    print(f"已started: {phase['name']}")


def complete_phase(phase_key):
    """完成某阶段"""
    data = load_progress()

    phase = data["phases"].get(phase_key)
    if not phase:
        print(f"错误: 阶段 '{phase_key}' 不存在")
        sys.exit(1)

    # 检查所有任务是否完成
    all_tasks = phase.get("tasks") or {}
    incomplete = [k for k, v in all_tasks.items() if v.get("status") != "completed"]
    if incomplete:
        print(f"警告: 以下任务未完成: {incomplete}")

    phase["status"] = "completed"
    phase["endTime"] = datetime.now().isoformat()
    save_progress(data)
    print(f"已完成: {phase['name']}")


def complete_task(phase_key, task_key):
    """完成单个任务"""
    data = load_progress()

    phase = data["phases"].get(phase_key)
    if not phase:
        print(f"错误: 阶段 '{phase_key}' 不存在")
        sys.exit(1)

    # 查找任务
    tasks = phase.get("tasks", {})
    if task_key not in tasks:
        print(f"错误: 任务 '{task_key}' 不存在")
        sys.exit(1)

    tasks[task_key]["status"] = "completed"
    save_progress(data)
    print(f"任务完成: {tasks[task_key]['description']}")

    # 检查是否所有任务都完成
    all_done = all(t.get("status") == "completed" for t in tasks.values())
    if all_done and phase.get("status") == "in_progress":
        print(f"建议: 阶段 {phase_key} 所有任务已完成，可以调用 complete {phase_key}")


def add_note(message):
    """添加备注"""
    data = load_progress()
    data["notes"].append({
        "time": datetime.now().isoformat(),
        "message": message
    })
    save_progress(data)
    print("备注已添加")


def check_dependencies(phase_key):
    """检查依赖是否满足"""
    data = load_progress()
    phase = data["phases"].get(phase_key)

    if not phase:
        print(f"错误: 阶段 '{phase_key}' 不存在")
        sys.exit(1)

    deps = phase.get("dependencies", [])
    if not deps:
        print(f"阶段 {phase_key} 没有依赖，可以开始")
        return True

    all_met = True
    for dep in deps:
        dep_phase = data["phases"].get(dep)
        if dep_phase:
            status = dep_phase.get("status")
            if status == "completed":
                print(f"✓ {dep}: 已完成")
            elif status == "in_progress":
                print(f"◐ {dep}: 进行中")
            else:
                print(f"✗ {dep}: {status}")
                all_met = False
        else:
            print(f"? {dep}: 不存在")
            all_met = False

    return all_met


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    command = sys.argv[1]

    if command == "read":
        read_progress()
    elif command == "start":
        if len(sys.argv) < 3:
            print("用法: python progress_manager.py start <phase>")
            sys.exit(1)
        start_phase(sys.argv[2])
    elif command == "complete":
        if len(sys.argv) < 3:
            print("用法: python progress_manager.py complete <phase>")
            sys.exit(1)
        complete_phase(sys.argv[2])
    elif command == "task":
        if len(sys.argv) < 4:
            print("用法: python progress_manager.py task <phase> <task>")
            sys.exit(1)
        complete_task(sys.argv[2], sys.argv[3])
    elif command == "note":
        if len(sys.argv) < 3:
            print("用法: python progress_manager.py note \"<message>\"")
            sys.exit(1)
        add_note(" ".join(sys.argv[2:]))
    elif command == "check":
        if len(sys.argv) < 3:
            print("用法: python progress_manager.py check <phase>")
            sys.exit(1)
        check_dependencies(sys.argv[2])
    else:
        print(f"未知命令: {command}")
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
