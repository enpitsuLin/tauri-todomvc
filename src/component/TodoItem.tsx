import { Todo } from '../types/todo'

const TodoItem: React.FC<{ todo: Todo }> = ({ todo }) => {
  return (
    <li>
      <div className="view">
        <input type="checkbox" className="toggle" checked={todo.done} autoFocus />
        <label>{todo.label}</label>
        <button className="destroy"></button>
      </div>
    </li>
  )
}
export default TodoItem
