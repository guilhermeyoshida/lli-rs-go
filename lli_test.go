package lli

import "testing"

func TestHelloToMyName(t *testing.T) {
	type args struct {
		name string
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			name: "hello to my name",
			args: args{name: "Gui"},
			want: "Hello, Gui!",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := HelloToMyName(tt.args.name); got != tt.want {
				t.Errorf("HelloToMyName() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestCountToken(t *testing.T) {
	type args struct {
		modelName string
		txt       string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test qtd tokens",
			args: args{
				modelName: "gpt-4",
				txt:       "Hello, my name is Gui",
			},
			want: 6,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := CountToken(tt.args.modelName, tt.args.txt); got != tt.want {
				t.Errorf("CountToken() = %v, want %v", got, tt.want)
			}
		})
	}
}
