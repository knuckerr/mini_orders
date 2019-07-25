export default {
  SETSUMMARY: (state, tables) => {
    state.tables = tables;
  },
  DELETETABLE: (state, id) => {
    state.tables = state.tables.filter(p => p.table.id != id);
  },
  DELETETABLES: (state, selectitems) => {
    state.tables = state.tables.filter(p => !selectitems.includes(p.table.id));
  },

  TABLESUMMARY: (state, table) => {
    if (table.length > 0) {
      state.table = table[0];
    }
  },

  NEWTABLE: (state, table) => {
    if (table.length > 0) {
      if (!state.tables.some(item => item.table.name === table[0].table.name)) {
        state.tables.push(table[0]);
      }
    }
  },
};
