export default {
  allTables: state => {
    return state.tables;
  },
  totalTables(state) {
    return state.tables.length;
  },
  tableSelectForm(state) {
    let tables = [];
    state.tables.forEach(item => {
      let table = item.table;
      tables.push({
        name: 'Room: ' + table.name,
        id: table.id,
        item_total: item.item_total,
        total: item.total,
        last_order: item.last_order,
        checked: false,
      });
    });
    return tables;
  },
};
