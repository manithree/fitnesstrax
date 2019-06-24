import React from "react"
import _ from "lodash/fp"
import moment from "moment-timezone"
import { connect } from "react-redux"

import { keyBy } from "../../common"
import RangeView from "../../components/range"
import DailyEntryView from "../../components/DailyEntry"
import Controller from "../../controller"
import Option from "../../option"
import * as redux from "../../redux"
import { Range, Record } from "../../types"
import { UserPreferences } from "../../userPrefs"

const bucketByDay = (recs: Array<Record>): Map<string, Array<Record>> =>
  keyBy((r: Record) => r.date.toFormat("yyyy-MM-dd"))(recs)

interface Props {
  controller: Controller
  history: Array<Record>
  prefs: UserPreferences
  range: Range
}

class History extends React.Component<Props, {}> {
  componentDidMount() {
    const { range } = this.props
    this.props.controller.fetchRecords(range)
  }

  render() {
    const { controller, history, prefs, range } = this.props

    const buckets = bucketByDay(history)
    _.entries(buckets).forEach(pair => console.log(pair[0].toString()))
    return (
      <div id="History">
        <RangeView classes={{}} range={range} />
        {_.compose(
          _.map(([k, r]: [string, Array<Record>]) => {
            return (
              <DailyEntryView
                key={k}
                date={k}
                prefs={prefs}
                records={r}
                saveRecords={records =>
                  this.props.controller.saveRecords(records)
                }
              />
            )
          }),
          _.sortBy(pair => pair[0]),
          _.entries,
        )(buckets)}
      </div>
    )
  }
}

const HistoryView = connect((state: redux.AppState) => ({
  prefs: redux.getPreferences(state),
  history: redux.getHistory(state),
}))(History)

export default HistoryView
