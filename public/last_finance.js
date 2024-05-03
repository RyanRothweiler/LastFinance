
export function show_error() {
		$('#exampleModal').modal('show');
}

export function build_graph(element_id, dat) {

		var myChart = echarts.init(document.getElementById(element_id));

		var data = [];
		for (let i = 0; i < dat.length; i++) {
				data.push([i, dat[i]]);
		}

		console.log(data);

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
