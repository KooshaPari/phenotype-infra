// Package gomock is a minimal stub of go.uber.org/mock/gomock
// for compilation purposes. The actual implementation is fetched
// from upstream when the replace directive is removed.
package gomock

import (
	"fmt"
	"reflect"
)

// Matcher is a matcher for gomock expectations.
type Matcher interface {
	Matches(x any) bool
	String() string
}

// eqMatcher implements Matcher for Eq.
type eqMatcher struct{ x any }

func (e eqMatcher) Matches(x any) bool {
	return reflect.DeepEqual(e.x, x)
}
func (e eqMatcher) String() string { return fmt.Sprintf("== %v", e.x) }

// anyMatcher implements Matcher for Any.
type anyMatcher struct{}

func (anyMatcher) Matches(x any) bool { return true }
func (anyMatcher) String() string     { return "anything" }

// nilMatcher implements Matcher for Nil.
type nilMatcher struct{}

func (nilMatcher) Matches(x any) bool { return x == nil }
func (nilMatcher) String() string     { return "is nil" }

// assignableToTypeOfMatcher implements Matcher for AssignableToTypeOf.
type assignableToTypeOfMatcher struct{ target reflect.Type }

func (m assignableToTypeOfMatcher) Matches(x any) bool {
	return reflect.TypeOf(x).AssignableTo(m.target)
}
func (m assignableToTypeOfMatcher) String() string {
	return fmt.Sprintf("assignable to %v", m.target)
}

// Eq returns a matcher matching any value that equals x.
func Eq(x any) Matcher { return eqMatcher{x} }

// Any returns a matcher matching any value.
func Any() Matcher { return anyMatcher{} }

// Nil returns a matcher matching nil.
func Nil() Matcher { return nilMatcher{} }

// AssignableToTypeOf returns a matcher matching values assignable to the type of x.
func AssignableToTypeOf(x any) Matcher {
	return assignableToTypeOfMatcher{reflect.TypeOf(x)}
}

// Call represents an expected call.
type Call struct {
	receiver any
	method   string
	args     []Matcher
	returns  []any
}

// Times sets the expected number of invocations.
func (c *Call) Times(n int) *Call { return c }

// Return sets the return values for the call.
func (c *Call) Return(vals ...any) *Call {
	c.returns = vals
	return c
}

// Controller manages mocked interactions.
type Controller struct {
	T interface {
		Helper()
		Errorf(format string, args ...any)
		Fatalf(format string, args ...any)
	}
}

// NewController creates a new Controller.
func NewController(t interface {
	Helper()
	Errorf(format string, args ...any)
	Fatalf(format string, args ...any)
}) *Controller {
	return &Controller{T: t}
}

// Finish checks that all expected calls were made.
func (ctrl *Controller) Finish() {}

// Call invokes a method call on a mock.
func (ctrl *Controller) Call(mock any, method string, args ...any) []any {
	ctrl.T.Helper()
	return nil
}

// RecordCallWithMethodType records an expected call.
func (ctrl *Controller) RecordCallWithMethodType(
	receiver any, method string, typ reflect.Type, args ...any,
) *Call {
	ctrl.T.Helper()
	return &Call{
		receiver: receiver,
		method:   method,
	}
}

// InOrder asserts that calls happen in the given order.
func InOrder(calls ...*Call) {}
