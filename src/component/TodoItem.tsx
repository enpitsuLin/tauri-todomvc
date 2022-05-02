import { Todo } from '../types/todo'

const TodoItem: React.FC<{ todo: Todo }> = () => {
  return (
    <li>
      <div className="view">
        <input type="checkbox" className="toggle" />
        <label>some text</label>
        <button className="destroy"></button>
      </div>
    </li>
  )
}
export default TodoItem
