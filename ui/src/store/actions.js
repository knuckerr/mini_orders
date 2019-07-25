import tables from '../api/tables';

export default {
  FetchSummaryTables: ({commit}) => {
    tables.GetSummaryTables()
      .then((res) => {
        commit('SETSUMMARY', res.data);
      })
      .catch(error => {
        console.error(error);
      });
  },
  FetchSummaryTable: ({commit}, name) => {
    tables.GetSummaryTable(name)
      .then(res => {
        commit('TABLESUMMARY', res.data);
      })

      .catch(error => {
        console.error(error);
      });
  },
  CreateTable: ({commit}, name) => {
    tables.newTable(name)
      .then(() => {
        tables.GetSummaryTable(name)
          .then(res => {
            commit('NEWTABLE', res.data);
          })

          .catch(error => {
            console.error(error);
          });
      })
      .catch(error => {
        console.error(error);
      });
  },
  RemoveTable: ({commit}, id) => {
    tables.DeleteTable(id)
      .then(() => {
        commit('DELETETABLE', id);
      })
      .catch(error => {
        console.error(error);
      });
  },

  RemoveTables: ({commit}, selectitems) => {
    tables.DeleteTables(selectitems)
      .then(() => {
        commit('DELETETABLES', selectitems);
      })
      .catch(error => {
        console.error(error);
      });
  },
};

