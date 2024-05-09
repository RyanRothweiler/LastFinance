
export function show_error() {
		$('#exampleModal').modal('show');
}

export function reload_page() {
		location.reload();
}

export function build_graph(element_id, dat) {

		var myChart = echarts.init(document.getElementById(element_id));

		var data = [];
		for (let i = 0; i < dat.length; i++) {
				data.push([i, dat[i]]);
		}

		const option = {
				tooltip: {
						trigger: 'axis',
				},
				xAxis: { },
				yAxis: { },
				series: [
						{
								data: data,
								type: 'line'
						}
				]
		};

		myChart.setOption(option);
}
